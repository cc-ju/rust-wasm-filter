.PHONY: release

release:
	cargo build --target wasm32-wasi --release