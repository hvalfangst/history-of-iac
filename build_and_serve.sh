#!/bin/bash

# Build the WASM package
wasm-pack build --target web --out-dir pkg --no-typescript

# Serve locally
echo "Starting local server at http://localhost:8000"
python3 -m http.server 8000
