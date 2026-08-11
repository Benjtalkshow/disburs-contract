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

# Everything CI enforces, in the order CI runs it.
ci:
	cargo fmt --all --check
	cargo clippy --locked --all-targets -- -D warnings
	cargo test --locked --all-targets
	stellar contract build

.PHONY: default build test fmt clippy clean ci
