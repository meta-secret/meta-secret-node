
docker run -ti -v "$(pwd)/wasm":/app meta-secret/web-cli-builder:latest sh -c "wasm-pack build --target web"