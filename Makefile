.PHONY: help credits

run: 
	cargo run -- parse $(filter-out $@,$(MAKECMDGOALS))

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy

precommit: test fmt clippy

help: 
	cargo run -- help

credits:
	cargo run -- credits