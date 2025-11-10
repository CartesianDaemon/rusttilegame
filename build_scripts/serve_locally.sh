#/bin/bash

# Do not 'set -e'. Browser will fail then retry.

cargo build --target wasm32-unknown-unknown # build debug wasm

explorer.exe # "http://localhost:4000"

basic-http-server test_wasm/
