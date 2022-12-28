#!/bin/bash
export GCP_PROJECT_ID=...

gcloud iam service-accounts create zkp-benchmark-manager --display-name "ZKP Benchmark Manager" --project=$GCP_PROJECT_ID
gcloud iam service-accounts keys create ./service-account-key.json --iam-account zkp-benchmark-manager@${GCP_PROJECT_ID}.iam.gserviceaccount.com
python3 -c 'import json, sys;json.dump(json.load(sys.stdin), sys.stdout)' < service-account-key.json > service-account-key.json
rm service-account-key.json
ACCOUNT=zkp-benchmark-manager@${GCP_PROJECT_ID}.iam.gserviceaccount.com
gcloud projects add-iam-policy-binding ${GCP_PROJECT_ID} --member=serviceAccount:${ACCOUNT} --role='roles/compute.admin'
gcloud projects add-iam-policy-binding ${GCP_PROJECT_ID} --member=serviceAccount:${ACCOUNT} --role='roles/storage.admin'