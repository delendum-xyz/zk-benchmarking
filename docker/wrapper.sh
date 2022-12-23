#!/bin/bash
cd /zk-benchmarking
git pull
./all.sh > $BENCH_OUTPUT_FILE
echo $GCP_SERVICE_ACCOUNT_JSON_KEY > /gcp_cred.json
gcloud auth activate-service-account --key-file /gcp_cred.json
gsutil cp $BENCH_OUTPUT_FILE gs://${GCP_BUCKET}
export NAME=$(curl -X GET http://metadata.google.internal/computeMetadata/v1/instance/name -H 'Metadata-Flavor: Google')
export ZONE=$(curl -X GET http://metadata.google.internal/computeMetadata/v1/instance/zone -H 'Metadata-Flavor: Google')
gcloud --quiet compute instances delete $NAME --zone=$ZONE