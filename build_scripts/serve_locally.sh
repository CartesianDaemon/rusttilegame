#/bin/bash

set -e

basic-http-server test_wasm/ &

explorer.exe "test_wasm\index.html"
