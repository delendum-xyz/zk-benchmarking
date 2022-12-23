#!/bin/bash
export INSTANCE_NAME=test-instance-1
export GCP_PROJECT_ID=delendum-zk-benchmark
export GCP_ZONE=us-west1-c
export MACHINE_TYPE=e2-highmem-2

gcloud compute instances create-with-container $INSTANCE_NAME --project=$GCP_PROJECT_ID --zone=$GCP_ZONE --machine-type=$MACHINE_TYPE --network-interface=network-tier=PREMIUM,subnet=default --maintenance-policy=MIGRATE --provisioning-model=STANDARD --image=projects/cos-cloud/global/images/cos-stable-101-17162-40-42 --boot-disk-size=10GB --boot-disk-type=pd-balanced --boot-disk-device-name=instance-1 --container-image=polymorpher/delendum-zk-benchmarking:v0.21-linux-x64 --container-restart-policy=always --container-env=BENCH_OUTPUT_FILE=$INSTANCE_NAME.log --container-env-file=.env --no-shielded-secure-boot --shielded-vtpm --shielded-integrity-monitoring --labels=container-vm=cos-stable-101-17162-40-42