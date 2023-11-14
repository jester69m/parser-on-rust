run: 
	cargo run

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy

precommit: test fmt clippy