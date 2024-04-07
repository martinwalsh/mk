#< Build automation for the `mk` command, an experimental GNU Make wrapper program.
#> Thanks for using `mk`!

include .makefiles/common.mk

VERSION := 0.3.0

CARGO ?= cargo

target/debug/mk: | _cmd_cargo
	$(CARGO) build --all --verbose --workspace $(FLAGS)

#| Install prerequisites
deps: | _cmd_cargo
	$(CARGO) install cargo-insta
.PHONY: deps

#| Run cargo build
#| Pass additional build flags using FLAGS
build: target/debug/mk
.PHONY: build

#| Build and install the `mk` command
install: target/debug/mk
	cp -iv target/debug/mk $(HOME)/.local/bin/mk
.PHONY: install

#| Uninstall the `mk` command
uninstall:
	rm -iv $(HOME)/.local/bin/mk
.PHONY: uninstall

#| Run cargo check and cargo test
#| Pass additional test flags with FLAGS
test: | lint
	$(CARGO) check --all
	$(CARGO) test --all --verbose --workspace $(FLAGS)
.PHONY: test

#| Run cargo clippy and cargo fmt --check
lint: | _cmd_cargo _fixme _todo
	$(CARGO) clippy --all
	$(CARGO) fmt --all -- --check
.PHONY: lint

#| Run cargo fmt
#| Use `FIX=yes` to automatically fix formatting issues
format: | _cmd_cargo
	$(CARGO) fmt --all $(if $(FIX),,-- --check)
.PHONY: format

#| Run cargo insta review
update-snapshots: | _cmd_cargo
	$(CARGO) insta review
.PHONY: update-snapshots

#| Bump the version and create new tag
#| Use `TO=0.4.0` to bump to a specific version
#| Use `DR=1` to perform a dry run (search and replace only)
bump: | _env_TO
	$(call sed_i) 's/^version = "$(VERSION)"/version = "$(TO)"/g' Cargo.toml
	$(call sed_i) 's/^VERSION := "$(VERSION)"/VERSION := $(TO)/g' Makefile
	$(call sed_i) 's/r\/$(VERSION)/r\/$(TO)/g' README.md
	$(CARGO) update
ifndef DR
	git add .
	git commit -m "Bump version to $(TO)"
	git tag $(TO)
	git push origin main --tags
endif
.PHONY: bump


#| Show this help message
help: | target/debug/mk
	@target/debug/mk --help
.PHONY: help
