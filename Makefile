#> The `mk` command wraps make targets, allowing you to run them from anywhere in the project.


target/debug/mk:
	cargo build --all

#> Run cargo build
build: target/debug/mk
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
#> Use `FIX=yes` to automatically fix formatting issues
format:
	cargo fmt --all $(if $(FIX),,-- --check)
.PHONY: format

#> Print this helpful message
help: | target/debug/mk
	@target/debug/mk --help
.PHONY: help

.DEFAULT_GOAL := help
