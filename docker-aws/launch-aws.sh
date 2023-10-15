#!/bin/bash
export INSTANCE_TYPE=g5.2xlarge

account_id=$(aws sts get-caller-identity --query "Account" --output text)

supported_architectures=$(aws ec2 describe-instance-types --instance-types $INSTANCE_TYPE --query "InstanceTypes[0].ProcessorInfo.SupportedArchitectures" --output text)
if [[ $supported_architectures == *"x86_64"* ]]; then
	image_id=$(aws ssm get-parameters --name /aws/service/ami-amazon-linux-latest/al2023-ami-kernel-default-x86_64 --query "Parameters[0].Value" --output text)
else
	image_id=$(aws ssm get-parameters --name /aws/service/ami-amazon-linux-latest/al2023-ami-kernel-default-arm64 --query "Parameters[0].Value" --output text)	
fi

instance_profile_arn="arn:aws:iam::${account_id}:instance-profile/benchmark-profile"

aws ec2 run-instances --image-id $image_id --instance-type $INSTANCE_TYPE --user-data file://user-data.sh --iam-instance-profile Arn=$instance_profile_arn --block-device-mapping "[ { \"DeviceName\": \"/dev/xvda\", \"Ebs\": { \"VolumeSize\": 20 } } ]"