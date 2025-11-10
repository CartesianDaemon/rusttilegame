#/bin/bash

# Do not 'set -e'. Browser will fail then retry.

cargo build --target wasm32-unknown-unknown # build debug wasm

# Need to switch to chrome without opening new file
# Or open file in new chrome eg. /mnt/c/Program\ Files/Google/Chrome/Application/chrome.exe http://localhost:4000
# explorer.exe # "http://localhost:4000"
/mnt/c/Program\ Files/Google/Chrome/Application/chrome.exe http://localhost:4000/programming_debug.html

basic-http-server test_wasm/
