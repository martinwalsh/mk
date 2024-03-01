#< Build automation for the `mk` command, an experimental GNU Make wrapper program.
#> Thanks for using `mk`!

-include .makefiles/common.mk

target/debug/mk:
	cargo build --all


#| Install prerequisites
deps:
	cargo install cargo-insta
.PHONY: deps

#| Run cargo build
build: target/debug/mk
.PHONY: build

#| Build and install the `mk` command
install: target/debug/mk
	cp -iv target/debug/mk $(HOME)/.local/bin/mk

#| Run cargo check and cargo test
test:
	cargo check --all
	cargo test --all
.PHONY: test


#| Run cargo insta review
snapshots:
	cargo insta review
.PHONY: snapshots

#| Run cargo clippy and cargo fmt --check
lint:
	cargo clippy --all
	cargo fmt --all -- --check
.PHONY: lint

#| Run cargo fmt
#| Use `FIX=yes` to automatically fix formatting issues
format:
	cargo fmt --all $(if $(FIX),,-- --check)
.PHONY: format

#| Print this helpful message
help: | target/debug/mk
	@target/debug/mk --help
.PHONY: help

.DEFAULT_GOAL := help
