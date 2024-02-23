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

- Make has been around a looooong time. It's stable, and probably installed in your dev env already.
- It can be used to automate any task that can be executed as a shell command.
- It provides primitives, like variables and dependencies, that can be used to reduce repetition and complexity.
- It has a standard interface so you can build multiple projects the same way, reducing cognitive load.
- Makefiles are self-documenting, by nature. Even more so after introducing `mk`.

### What is `mk`?

I'm using this project as an opportunity to learn the [Rust](https://www.rust-lang.org/)
programming language. So you should keep in mind that I'm a beginner.

Nonetheless, I believe you will find `mk` useful whether GNU Make is already
part of your project, or you are just beginning a new one.

The `mk` command is intended to offer a minimal set of additional convenience
features absent from GNU Make itself.

- Execution from any directory in a software project
- Automated generation of `help` text

#### Makefile discovery

The GNU make command requires that you execute commands at the root of your
project, which can be cumbersome.

The `mk` command, on the other hand, will traverse the filesystem upward until
it locates a `Makefile` -- when executed anywhere in the project -- and then
execute the corresponding `make` command with the supplied arguments, in the
directory where the `Makefile` was found.

#### Automated help text

By default, the `mk` command will parse your `Makefile` and generate help text
for all of the make targets discovered. For example:

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
...
Usage: mk [OPTIONS] [COMMAND]

Commands:
  build
  test
...
```

With the addition of comments prefixed with `#>`, placed above a target
definition, the `mk` command will include them in the help output. For example:

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
#> Use FIX=yes to automatically fix files
format:
    cargo fmt $(if $(FIX),,--check)
```

```sh
$ mk --help
...
Usage: mk [OPTIONS] [COMMAND]

Commands:
  format  Runs cargo fmt
              Use FIX=yes to automatically fix files
...

```

## Installation

To install, visit the [releases](https://github.com/martinwalsh/mk/releases)
page, download the appropriate version for your system, extract the `mk` binary,
and add it to your system's `PATH`.

Alternatively, you can execute the following in a terminal:

```sh
curl -sSL https://mk.brightops.dev/r/latest | bash
```

## Usage

The following usage examples rely on the [`Makefile`](./Makefile) in this project, but the
examples apply to any project containing a `Makefile`.

Running `mk` without arguments, produces `--help` output.

```sh
$ mk
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

Running `mk` in a subfolder will execute `make` in the project's root directory.


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
              Use `FIX=yes` to automatically fix formatting issues
  help    Print this helpful message
...
```

Running `mk build`, in any directory of the project, will run `make build` in
the project's root. This works for any target defined in the project's `Makefile`.

```sh
~/src/mk $ mk build # `mk build` executes `make build`
cargo build --all
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
```

The `mk` command can be used to create or replace an existing `help` target. Makeception!

```Makefile
help:
    @mk --help
```

```sh
$ make help  # or mk help, `mk -h`, or `mk --help`
...
Usage: mk [OPTIONS] [COMMAND]

Commands:
  build   Run cargo build
  test    Run cargo check and cargo test
...
```
