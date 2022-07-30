
docker run -ti --rm -v "$(pwd)/wasm":/app meta-secret/web-cli-builder:latest sh -c "wasm-pack build --target web --dev"
rm -rf ui/pkg
cp -R wasm/pkg ui
