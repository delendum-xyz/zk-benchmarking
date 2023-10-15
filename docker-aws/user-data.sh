#!/bin/bash
aws_bucket="zk-benchmarking" # Update this to your S3 bucket

instance_type=$(TOKEN=`curl -X PUT "http://169.254.169.254/latest/api/token" -H "X-aws-ec2-metadata-token-ttl-seconds: 21600"` \
&& curl -H "X-aws-ec2-metadata-token: $TOKEN" http://169.254.169.254/latest/meta-data/instance-type)
bench_output_file=${instance_type}.log

yum update -y
yum install docker -y
service docker start

account_id=$(aws sts get-caller-identity --query "Account" --output text)
current_region=$(TOKEN=`curl -X PUT "http://169.254.169.254/latest/api/token" -H "X-aws-ec2-metadata-token-ttl-seconds: 21600"` \
&& curl -H "X-aws-ec2-metadata-token: $TOKEN" http://169.254.169.254/latest/meta-data/placement/region)

# Login Docker
aws ecr get-login-password | docker login --username AWS --password-stdin ${account_id}.dkr.ecr.${current_region}.amazonaws.com

docker pull ${account_id}.dkr.ecr.${current_region}.amazonaws.com/zk-benchmarking
docker run -e AWS_BUCKET=$aws_bucket -e BENCH_OUTPUT_FILE=$bench_output_file --name benchmark ${account_id}.dkr.ecr.${current_region}.amazonaws.com/zk-benchmarking

# Terminate current instance
instance_id=$(TOKEN=`curl -X PUT "http://169.254.169.254/latest/api/token" -H "X-aws-ec2-metadata-token-ttl-seconds: 21600"` \
&& curl -H "X-aws-ec2-metadata-token: $TOKEN" http://169.254.169.254/latest/meta-data/instance-id)
aws ec2 terminate-instances --instance-ids $instance_id