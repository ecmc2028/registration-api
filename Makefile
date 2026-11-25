.PHONY: test

test:
	cargo test -p shared && wasm-pack test --headless --chrome crates/shared
