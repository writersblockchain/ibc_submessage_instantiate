.PHONY: build-mainnet _build-mainnet compress-wasm build-mainnet-reproducible

SUBDIR := contracts/gateway-simple
ROOT_DIR := ..

build-mainnet: _build-mainnet compress-wasm

_build-mainnet:
	cd $(SUBDIR) && RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown

compress-wasm:
	cp $(SUBDIR)/target/wasm32-unknown-unknown/release/*.wasm $(ROOT_DIR)/optimized-wasm/
	docker run --rm -v $(ROOT_DIR)/optimized-wasm:/optimized \
		--mount type=volume,source=wasm_cache,target=/usr/local/cargo/registry \
		mr7uca/wasm-contract-optimizer:0.0.10

build-mainnet-reproducible:
	docker run --rm -v "$$(pwd)":/contract \
		--mount type=volume,source="$$(basename "$$(pwd)")_cache",target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		mr7uca/wasm-contract-optimizer:0.0.10
