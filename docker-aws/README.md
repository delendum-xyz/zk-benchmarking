# Benchmarking on AWS

This folder contains scripts for 

1. Building the docker image for automated benchmarking on AWS
2. Setting up AWS infrastructure and service accounts for benchmarking
3. Launching a single or a group of benchmarking self-terminating instances
   
The benchmarking results are automatically stored in an AWS S3 bucket of your choice, each named with the instance type used.

## Docker images used
- **v0.11-linux-x64**: Extended from v0.1-linux-x64, with Google Cloud Computing Platform tools installed (gcloud, gsutil).
- **Local Dockerfile**: Extended from v0.11-linux-x64, with AWS CLI installed.

## Steps
### Prerequisites
Make sure you have AWS CLI already [installed](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) and [configured](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-configure.html) on your computer and terminal. Make sure that you have admin credentials configured.

### Prepare the AWS infrastructure
1. Run Docker daemon
2. [Create an AWS S3 bucket on your own](https://docs.aws.amazon.com/AmazonS3/latest/userguide/create-bucket-overview.html)
3. Update the variable `aws_bucket` in `user-data.sh` to your bucket name
4. Run the script `[init-setup.sh](init-setup.sh)` for the rest of the infrastructure.

### Single instance benchmarking

Edit `[launch-aws.sh](launch-aws.sh)` and change `INSTANCE_TYPE` before you run the script.

### Batch benchmarking

Edit `[launch-aws-batch.sh](launch-aws-batch.sh)` and change `INSTANCE_TYPES` before you run the script.

### Collecting the results

The simplest way is to [go to your bucket](https://s3.console.aws.amazon.com/s3/buckets) and download the results using the browser GUI.

Some example outputs are in [example-output](./example-output) .