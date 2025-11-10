#/bin/bash

# Do not 'set -e'. Browser will fail then retry.

explorer.exe "http://localhost:4000"

basic-http-server test_wasm/
