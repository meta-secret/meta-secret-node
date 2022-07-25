
docker run -ti -v "$(pwd)/web-cli-wasm":/app meta-secret/web-cli-builder:latest sh -c "wasm-pack build"