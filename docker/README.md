# Benchmarking on GCP

This folder contains scripts for 

1. building the docker images for automated benchmarking on GCP
2. setting up a GCP project and service accounts for benchmarking
3. launching groups of benchmarking self-terminating instances

The benchmarking results are automatically stored in a GCP storage bucket, each tagged with the machine type of the instance used

## Building docker images

You can skip this section if you are only interested in running the benchmarking on cloud. A few pre-built docker images are already uploaded on Docker Hub under [polymorpher/delendum-zk-benchmarking](https://hub.docker.com/repository/docker/polymorpher/delendum-zk-benchmarking):

- **v0.1-linux-x64**: Debian OS preloaded with rust, ssl, git, and standard build tools, and the benchmarking repository. Suitable for testing locally
- **v0.11-linux-x64**: Extended from v0.1-linux-x64, with Google Cloud Computing Platform tools installed (gcloud, gsutil)
- **v0.21-linux-x64**: Extended from v0.11-linux-x64, suitable for deployment as a container image under a GCP VM Instance. The instance will automatically pull the latest code from this repository, build all rust dependencies, compile everything, run all benchmarking, logs to a file, and upload the file to a pre-configured bucket in Google Storage. After this is all done, the instance will self-terminate so it doesn't incur any more billing than necessary. 

You can build **v0.21-linux-x64** using `build.sh` in this folder

## Prepare the GCP project

First, make sure your project already has [Compute Engine API](https://console.cloud.google.com/compute/instances) enabled, and you have Google Cloud CLI already [installed](https://cloud.google.com/sdk/docs/install) and [configured](https://cloud.google.com/sdk/docs/initializing) on your computer and terminal.

The next step is to create a service account for managing the cloud benchmarking instances and to upload the results to Google Storage buckets. A script `[make-service-account.sh](make-service-account.sh)` is already prepared for you. Just replace the value of `GCP_PROJECT_ID=...` in the script with your project ID.

You will see a file `service-account-key-minified.json` generated after you finish running the script. We will use it in the next section.

## (Batch) creating benchmarking instances

You need to configure a few environment variables before you can do this.

1. Create a copy of `.env.example` and name it `.env`
2. Replace the value of `GCP_SERVICE_ACCOUNT_JSON_KEY=...` with the content in `service-account-key-minified.json` (it will become a very long line!)
3. Fill in the rest:
   - `GCP_BUCKET` can be any bucket, but you need to [create the bucket](https://console.cloud.google.com/storage/browser) first in your project
   - `BENCH_OUTPUT_FILE` is the name of the file you want to dump into the bucket. It can be any valid filename. It will only be relevant in non-batch mode. In batch mode, the script will override this variable with VM instance name.

### Single instance benchmarking

Edit `[launch-gcp.sh](launch-gcp.sh)` and change the variables based on your GCP project setting, before you run the script

- `INSTANCE_NAME`: the name of the GCP instance
- `GCP_PROJECT_ID`: your GCP project ID
- `GCP_ZONE`: where you want the instance to be hosted, see [this list](https://cloud.google.com/compute/docs/regions-zones)
- `MACHINE_TYPE`: choose one from [this list](https://gcpinstances.doit-intl.com/). You need at least 12GB of memory to finish the benchmarking, otherwise some jobs will be preemptively killed due to out-of-memory error 

### Batch benchmarking

Edit `[launch-gcp-batch.sh](launch-gcp-batch.sh)` and change the variables based on your GCP project setting, before you run the script

- `INSTANCE_NAME_BASE`: the prefix of each VM instance's name
- `GCP_PROJECT_ID`: same as above
- `GCP_ZONE`: same as above
- `MACHINE_TYPES`: the array of instance types you want to run benchmarking on. Make sure you have [enough quota](https://cloud.google.com/docs/quota) before you run it

### Collecting the results

The simplest way is to [go to your bucket](https://console.cloud.google.com/storage/browser) and download the results using the browser GUI.

Some example outputs are in [example-output](./example-output) folder