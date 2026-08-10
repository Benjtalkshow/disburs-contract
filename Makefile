default: build

build:
	stellar contract build

test:
	cargo test

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets -- -D warnings

clean:
	cargo clean
