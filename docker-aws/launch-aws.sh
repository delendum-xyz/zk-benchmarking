#!/bin/bash
export INSTANCE_TYPE=t3.xlarge

account_id=$(aws sts get-caller-identity --query "Account" --output text)
image_id=$(aws ssm get-parameters --name /aws/service/ami-amazon-linux-latest/al2023-ami-kernel-default-x86_64 --query "Parameters[0].Value" --output text)
instance_profile_arn="arn:aws:iam::${account_id}:instance-profile/benchmark-profile"

aws ec2 run-instances --image-id $image_id --instance-type $INSTANCE_TYPE --user-data file://user-data.sh --iam-instance-profile Arn=$instance_profile_arn