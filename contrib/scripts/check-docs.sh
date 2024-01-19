#!/bin/bash

# Needed to exit from script on error
set -e

buildargs=(
    "-p webln --target wasm32-unknown-unknown"
)

for arg in "${buildargs[@]}"; do
    echo  "Checking '$arg' docs"
    cargo doc $arg --all-features
    echo
done