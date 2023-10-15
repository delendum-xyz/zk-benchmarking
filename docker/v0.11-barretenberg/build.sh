#!/bin/bash
cd ../../barretenberg/cpp/
docker build -f dockerfiles/Dockerfile.x86_64-linux-clang-benchmarks . --platform linux/amd64 -t barretenberg/external_benchmarks
cd ../../docker/v0.11-barretenberg
docker build . --platform linux/amd64 -t polymorpher/delendum-zk-benchmarking:v0.11-barretenberg-linux-x64
