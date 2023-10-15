export repository_name="zk-benchmarking"
export account_id=$(aws sts get-caller-identity --query "Account" --output text)
export current_region=$(aws configure get region)

# Push local image to ECR
aws ecr create-repository --repository-name $repository_name
aws ecr get-login-password | docker login --username AWS --password-stdin $account_id.dkr.ecr.$current_region.amazonaws.com
docker build -t $account_id.dkr.ecr.$current_region.amazonaws.com/$repository_name --platform linux/amd64 .
docker push $account_id.dkr.ecr.$current_region.amazonaws.com/$repository_name

# Set up instance profile for EC2 instances
aws iam create-role --role-name benchmark-role --assume-role-policy-document file://trust-policy.json
aws iam attach-role-policy --policy-arn arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryReadOnly --role-name benchmark-role
aws iam attach-role-policy --policy-arn arn:aws:iam::aws:policy/AmazonS3FullAccess --role-name benchmark-role
aws iam attach-role-policy --policy-arn arn:aws:iam::aws:policy/AmazonEC2FullAccess --role-name benchmark-role
aws iam create-instance-profile --instance-profile-name benchmark-profile
aws iam add-role-to-instance-profile --role-name benchmark-role --instance-profile-name benchmark-profile