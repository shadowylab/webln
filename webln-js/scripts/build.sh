#!/bin/bash

set -e

cd $(dirname "$0")/..

wasm-pack build --target web --no-pack --scope shadowylab --weak-refs --out-dir pkg "${WASM_PACK_ARGS[@]}"

# Shrinking .wasm Size
wc -c pkg/webln_js_bg.wasm