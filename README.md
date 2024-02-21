# mk

## Introduction

`mk` is an experimental wrapper for the ubiquitous GNU make, widely-used for
automating routine tasks in countless software projects. `mk` is meant to
enhance the functionality of GNU make by allowing the execution of make commands
from any directory within a project, as opposed to the project's root alone.

Additionally, `mk` will parse a project's Makefile and generate help text. Just
write a comment in your Makefile prefixed with `#>` to describe the target that
follows, and `mk` will include it in the output of `mk --help` automatically.

`mk` is designed to be used in any project that already uses GNU make, or in
new projects just getting started. It's a versatile tool that can simplify and
enhance your use of Makefiles.

`mk` was created to fill in gaps in the functionality of GNU make, encountered
over years of productive use, and also as a way to learn the Rust programming
language.


## Installation

Head over to the releases page and download the archive for your operating
system and CPU arch, extract the `mk` binary from the archive and place it
somewhere in your system `PATH`.


## Usage

The following examples use the [`Makefile`](`Makefile`) in this project. But it
will work the same in any project that contains a `Makefile`.

```zsh
❯ mk # used without arguments, `mk` produces help output
An experimental command-line wrapper of GNU Make.

Usage: mk [OPTIONS] [COMMAND]

Commands:
  build   Run cargo build
  test    Run cargo test
  lint    Run cargo clippy and cargo fmt --check
  format  Run cargo fmt
              Use `CHECK=1` to check formatting without modifying files
  help    Print this helpful message

Options:
  -m, --me       Run an internal `mk` command
  -h, --help     Print help
  -V, --version  Print version

The `mk` command wraps make targets, allowing you to run them from anywhere in the project.
```

```zsh
~/src/mk
❯ mkdir -p subfolder/subfolder/subfolder
~/src/mk
❯ cd subfolder/subfolder/subfolder 
~/src/mk/subfolder/subfolder/subfolder
❯ mk -h # `mk -h` or `mk --help` do the same thing
❯ .     #  even from a sub-folder in your project
An experimental command-line wrapper of GNU Make.

Usage: mk [OPTIONS] [COMMAND]

Commands:
  build   Run cargo build
  test    Run cargo test
  lint    Run cargo clippy and cargo fmt --check
  format  Run cargo fmt
              Use `CHECK=1` to check formatting without modifying files
  help    Print this helpful message

Options:
  -m, --me       Run an internal `mk` command
  -h, --help     Print help
  -V, --version  Print version

The `mk` command wraps make targets, allowing you to run them from anywhere in the project.
```

```zsh
~/src/mk
❯ mk build # `mk build` executes `make build`
cargo build --all
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
```
