default: build

build:
	stellar contract build
	@ls -l ../../target/wasm32v1-none/release/*.wasm 2>/dev/null || true

test:
	cargo test

clean:
	cargo clean
