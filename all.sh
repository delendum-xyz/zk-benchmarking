#!/bin/bash

run_benchmark () {
    echo "--------------------------------------------------"
    echo "Start: $1"
    echo "--------------------------------------------------"
    pushd $2
    $3
    popd
    echo "--------------------------------------------------"
    echo "Done: $1"
    echo "--------------------------------------------------"
    echo ""
    echo ""
}

######################################################################
#             Team              Directory       Command
run_benchmark "Polygon Miden"   "miden"         "cargo run --release"
run_benchmark "RISC Zero"       "risczero"      "cargo run --release"
