#!/bin/bash

# Needed to exit from script on error
set -e

# MSRV
msrv="1.64.0"

is_msrv=false
version=""

# Check if "msrv" is passed as an argument
if [[ "$#" -gt 0 && "$1" == "msrv" ]]; then
    is_msrv=true
    version="+$msrv"
fi

# Check if MSRV
if [ "$is_msrv" == true ]; then
    # Install MSRV
    rustup install $msrv
    rustup component add clippy --toolchain $msrv
    rustup target add wasm32-unknown-unknown --toolchain $msrv
fi

buildargs=(
    "-p webln --target wasm32-unknown-unknown"
    "-p webln --no-default-features --target wasm32-unknown-unknown"
)

for arg in "${buildargs[@]}"; do
    if [[ $version == "" ]]; then
        echo  "Checking '$arg' [default]"
    else
        echo  "Checking '$arg' [$version]"
    fi
    cargo $version check $arg
    cargo $version clippy $arg -- -D warnings
    echo
done