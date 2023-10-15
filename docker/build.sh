#!/bin/bash
docker build . -f ./Dockerfile-barretenberg --platform linux/amd64 -t polymorpher/delendum-zk-benchmarking:v0.21-barretenberg-linux-x64
docker build . --platform linux/amd64 -t polymorpher/delendum-zk-benchmarking:v0.21-linux-x64
