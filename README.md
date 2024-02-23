# mk

## Introduction

Welcome makers! This project introduces an experimental convenience wrapper for
GNU make, named `mk`.

[GNU Make](https://www.gnu.org/software/make/) is a widely used command-line
tool for automating routine tasks from instructions defined in a `Makefile`,
designed to manage software in compiled languages, like C or C++, allowing the
efficient handling of dependencies between source files and avoiding unnecessary
compilation. But the ubiquity of make and its straightforward syntax make it an
ideal choice as a general-purpose automation tool. As a result, it is present in
countless software projects of all kinds.

#### Benefits of GNU Make

- Make has been around a looooong time. It's stable, and may be installed in your dev environment already.
- It can be used to automate any task that can be executed as a shell command.
- Make provides primitives, like variables, dependencies and pattern rules, that can be used to reduce repetition and complexity.
- Make provides a standard interface so that you can build multiple projects the same way, reducing cognitive load.
- Makefiles are self-documenting by nature, and significantly improved by introducing `mk`.

### What is `mk`?

I'm using this project as an opportunity to learn the
[Rust](https://www.rust-lang.org/) programming language. Therefore, you should
keep in mind that I'm a beginner here. Nonetheless, you may find it a useful
tool whether you're already using GNU Make, or just starting a new project.

The `mk` wrapper program is intended to offer additional convenience features
absent from GNU Make itself.

- Automated generation of `help` text
- Makefile discovery within any directory of a project

#### Automated help text

By default, the `mk` command will parse your `Makefile` and generate help text for all of the make targets discovered.
For example:

```Makefile
# Makefile
build:
    @echo "building ..."

test:
    @echo "testing ..."

.PHONY: build test

```

```sh
$ mk --help
An experimental command-line wrapper for GNU Make.

Usage: mk [OPTIONS] [COMMAND]

Commands:
  build
  test

Options:
      --me       Run an internal `mk` command
  -h, --help     Print help
  -V, --version  Print version

```

With the addition of comments prefixed with `#>` prior to a target definition, the `mk` command will include them in the help output. For example:

```Makefile
# Makefile

#> Builds the project
build:
    @echo "building ..."

#> Runs the tests
test:
    @echo "testing ..."

.PHONY: build test

```

```sh
...

Usage: mk [OPTIONS] [COMMAND]

Commands:
  build  Builds the project
  test   Runs the tests

...
```

You can provide multiple lines of special-purpose comments, and they will be reflected in the generated help text.

```Makefile
#> Runs cargo fmt
#> Use FIX=1 to automatically fix files
format:
    cargo fmt $(if $(FIX),,--check)
```

```sh
$ mk --help
...
Usage: mk [OPTIONS] [COMMAND]

Commands:
  format  Runs cargo fmt
              Use FIX=1 to automatically fix files
...

```

#### Makefile discovery

The GNU make command requires that you are running in a terminal at the root of
your project, which can be cumbersome.

When executed within your project, the `mk` command will traverse the filesystem
upward until it locates a `Makefile`, and then execute a `make` command with the
supplied arguments in the same directory.

## Installation

To install, visit the [releases](releases/) page, download the appropriate version for your
system, extract the `mk` binary, and add it to your system's `PATH`.

## Usage

The following examples use the [`Makefile`](`Makefile`) in this project, but the
same concepts apply to any project that contains a `Makefile`.

```sh
$ mk # used without arguments, `mk` produces help output
...
Usage: mk [OPTIONS] [COMMAND]

Commands:
  build   Run cargo build
  test    Run cargo test
  lint    Run cargo clippy and cargo fmt --check
  format  Run cargo fmt
              Use `CHECK=1` to check formatting without modifying files
  help    Print this helpful message
...
The `mk` command wraps make targets, allowing you to run them from anywhere in the project.
```

```sh
~/src/mk $ mkdir -p subfolder/subfolder/subfolder
~/src/mk $ cd subfolder/subfolder/subfolder 
~/src/mk/subfolder/subfolder/subfolder $ mk -h    # or `mk --help`
...
Usage: mk [OPTIONS] [COMMAND]

Commands:
  build   Run cargo build
  test    Run cargo test
  lint    Run cargo clippy and cargo fmt --check
  format  Run cargo fmt
              Use `CHECK=1` to check formatting without modifying files
  help    Print this helpful message
...
```

```sh
~/src/mk $ mk build # `mk build` executes `make build`
cargo build --all
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
```
