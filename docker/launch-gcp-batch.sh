#!/bin/bash
export INSTANCE_NAME_BASE=test-batch-instance
export RUST_BENCHMARKS_SUFFIX=rust
export BARRETENBERG_BENCHMARKS_SUFFIX=barretenberg
export GCP_PROJECT_ID=delendum-zk-benchmark
export GCP_ZONE=us-west1-c
export MACHINE_TYPES=("e2-highmem-2" "e2-standard-4" "e2-highmem-4" "e2-standard-8")

for MACHINE_TYPE in "${MACHINE_TYPES[@]}"; do
  export INSTANCE_NAME=${INSTANCE_NAME_BASE}-${MACHINE_TYPE}-${RUST_BENCHMARKS_SUFFIX}
  gcloud compute instances create-with-container $INSTANCE_NAME --project=$GCP_PROJECT_ID --zone=$GCP_ZONE --machine-type=$MACHINE_TYPE --network-interface=network-tier=PREMIUM,subnet=default --maintenance-policy=MIGRATE --provisioning-model=STANDARD --image=projects/cos-cloud/global/images/cos-stable-101-17162-40-42 --boot-disk-size=25GB --boot-disk-type=pd-balanced --boot-disk-device-name=instance-1 --container-image=polymorpher/delendum-zk-benchmarking:v0.21-linux-x64 --container-restart-policy=always --container-env=BENCH_OUTPUT_FILE=$INSTANCE_NAME.log --container-env-file=.env --no-shielded-secure-boot --shielded-vtpm --shielded-integrity-monitoring --labels=container-vm=cos-stable-101-17162-40-42
done

for MACHINE_TYPE in "${MACHINE_TYPES[@]}"; do
  export INSTANCE_NAME=${INSTANCE_NAME_BASE}-${MACHINE_TYPE}-${BARRETENBERG_BENCHMARKS_SUFFIX}
  gcloud compute instances create-with-container $INSTANCE_NAME --project=$GCP_PROJECT_ID --zone=$GCP_ZONE --machine-type=$MACHINE_TYPE --network-interface=network-tier=PREMIUM,subnet=default --maintenance-policy=MIGRATE --provisioning-model=STANDARD --image=projects/cos-cloud/global/images/cos-stable-101-17162-40-42 --boot-disk-size=25GB --boot-disk-type=pd-balanced --boot-disk-device-name=instance-1 --container-image=polymorpher/delendum-zk-benchmarking:v0.21-barretenberg-linux-x64 --container-restart-policy=always --container-env=BENCH_OUTPUT_FILE=$INSTANCE_NAME.log --container-env-file=.env --no-shielded-secure-boot --shielded-vtpm --shielded-integrity-monitoring --labels=container-vm=cos-stable-101-17162-40-42
d