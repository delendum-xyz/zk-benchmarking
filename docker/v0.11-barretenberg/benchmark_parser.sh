#!/bin/bash

CSV_OUTPUT=$(mktemp)
sha256_proof_1_iteration_name="generate_sha256_proof_bench/0"
sha256_proof_10_iterations_name="generate_sha256_proof_bench/1"
sha256_proof_100_iterations_name="generate_sha256_proof_bench/2"
sha256_verification_1_iteration_name="verify_sha256_proof_bench/0"
sha256_verification_10_iterations_name="verify_sha256_proof_bench/1"
sha256_verification_100_iterations_name="verify_sha256_proof_bench/2"
blake3s_proof_1_iteration_name="generate_blake3s_proof_bench/0"
blake3s_proof_10_iterations_name="generate_blake3s_proof_bench/1"
blake3s_proof_100_iterations_name="generate_blake3s_proof_bench/2"
blake3s_verification_1_iteration_name="verify_blake3s_proof_bench/0"
blake3s_verification_10_iterations_name="verify_blake3s_proof_bench/1"
blake3s_verification_100_iterations_name="verify_blake3s_proof_bench/2"

# Proof size for no public inputs is always 2144, you can check by running the benchmarks with info uncommented
# Unfortunately, google benchmarks don't allow putting additional info into the output of the benchmark
proof_size=2144

get_time_measurement() {
    local test_name="$1"
    local measurement=$(cat "$CSV_OUTPUT" | grep "$test_name" | cut -d , -f3)
    local unit_of_measurement=$(cat "$CSV_OUTPUT" | grep "$test_name" | cut -d , -f5)
    echo "${measurement}${unit_of_measurement}"
}

./bin/external_bench --benchmark_format=csv >$CSV_OUTPUT

START_TIME=$(date +"%Y%m%d%H%M%S")
echo "Start time: $START_TIME"
echo ""
echo "--------------------------------------------------"
echo "Start: Barretenberg"
echo "--------------------------------------------------"
echo $(pwd)
echo ""
echo "+ begin job_number:   0 iter_blake3"
echo "+ job_name:           \"iter_blake3\""
echo "+ job_size:           1"
echo "+ proof_duration:     $(get_time_measurement $blake3s_proof_1_iteration_name)"
echo "+ verify_duration:    $(get_time_measurement $blake3s_verification_1_iteration_name)"
echo "+ proof_bytes:        $proof_size"
echo "+ end job_number:     0"
echo ""

echo "+ begin job_number:   1 iter_blake3"
echo "+ job_name:           \"iter_blake3\""
echo "+ job_size:           10"
echo "+ proof_duration:     $(get_time_measurement $blake3s_proof_10_iterations_name)"
echo "+ verify_duration:    $(get_time_measurement $blake3s_verification_10_iterations_name)"
echo "+ proof_bytes:        $proof_size"
echo "+ end job_number:     1"
echo ""

echo "+ begin job_number:   2 iter_blake3"
echo "+ job_name:           \"iter_blake3\""
echo "+ job_size:           100"
echo "+ proof_duration:     $(get_time_measurement $blake3s_proof_100_iterations_name)"
echo "+ verify_duration:    $(get_time_measurement $blake3s_verification_100_iterations_name)"
echo "+ proof_bytes:        $proof_size"
echo "+ end job_number:     2"
echo "-jobs:                3"
echo "-done"
echo ""
echo "+ begin job_number:   0 iter_sha2"
echo "+ job_name:           \"iter_sha2\""
echo "+ job_size:           1"
echo "+ proof_duration:     $(get_time_measurement $sha256_proof_1_iteration_name)"
echo "+ verify_duration:    $(get_time_measurement $sha256_verification_1_iteration_name)"
echo "+ proof_bytes:        $proof_size"
echo "+ end job_number:     0"
echo ""

echo "+ begin job_number:   1 iter_sha2"
echo "+ job_name:           \"iter_sha2\""
echo "+ job_size:           10"
echo "+ proof_duration:     $(get_time_measurement $sha256_proof_10_iterations_name)"
echo "+ verify_duration:    $(get_time_measurement $sha256_verification_10_iterations_name)"
echo "+ proof_bytes:        $proof_size"
echo "+ end job_number:     1"
echo ""

echo "+ begin job_number:   2 iter_sha2"
echo "+ job_name:           \"iter_sha2\""
echo "+ job_size:           100"
echo "+ proof_duration:     $(get_time_measurement $sha256_proof_100_iterations_name)"
echo "+ verify_duration:    $(get_time_measurement $sha256_verification_100_iterations_name)"
echo "+ proof_bytes:        $proof_size"
echo "+ end job_number:     2"
echo "-jobs:                3"
echo "-done"

# Using almost the same format as standard benchmarks, but we have milliseconds
echo "--------------------------------------------------"
echo "Done: Barretenberg"
echo "--------------------------------------------------"
