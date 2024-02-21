#> The `mk` command wraps make targets, allowing you to run them from anywhere in the project.

#> Run cargo build
build:
	cargo build --all
.PHONY: build

#> Run cargo check and cargo test
test:
	cargo check --all
	cargo test --all
.PHONY: test

#> Run cargo clippy and cargo fmt --check
lint:
	cargo clippy --all
	cargo fmt --all -- --check
.PHONY: lint

#> Run cargo fmt
#> Use `CHECK=1` to check formatting without modifying files
format:
	cargo fmt --all $(if $(CHECK),-- --check)
.PHONY: format

#> Print this helpful message
help:
	@mk --help
.PHONY: help

.DEFAULT_GOAL := help
