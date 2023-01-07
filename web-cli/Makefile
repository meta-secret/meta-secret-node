
clean:
	rm -rf ui/src/model
	rm -rf ../meta-secret-core/schema/target

build:
	cd wasm && wasm-pack build --target web --dev

	rm -rf ui/pkg
	cp -R wasm/pkg ui

	cd ui && npm run dev

generate_typescript_models:
	cd ../meta-secret-core/schema && make generate_schema_type_script
	cp -r  ../meta-secret-core/schema/target/core-models-ts/model ui/src
