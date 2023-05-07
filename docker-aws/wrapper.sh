#!/bin/bash
cd /zk-benchmarking
git pull
./all.sh > $BENCH_OUTPUT_FILE
aws s3 cp $BENCH_OUTPUT_FILE s3://${AWS_BUCKET}