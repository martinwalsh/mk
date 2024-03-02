# mk-makefiles includes

> [!IMPORTANT]
> The information on this page is not yet implemented.

The external project, called [`mk-makefiles`](https://github.com/brightops/mk-makefiles),
provides a few helpful shortcuts for inclusion in a general-purpose `Makefile`, in addition to
programming language or tooling specific templates.

## Enable includes

The `mk` command integrates the installation of `mk-makefiles` with the `--me`
flag. For example, to install the common set of helpers into your project,
simply execute `mk --me include`.

This will create a folder named `.makefiles` in the root directory of the
project, and add an include at `.makefiles/common.mk` with the common helpers.

## Add a language specific include

Once you have the common helpers installed, you can add an include for a
particular programming language or tool. Simply add an include directive, after
`include .makefiles/common.mk`. For example:


```Makefile
include .makefiles/common.mk # This include directive and the file it points to should already exist.
include .makefiles/python-poetry.mk
```

Then run `mk` or `make` to install the language-specific include. This relies on
the default behavior of GNU Make, with a target in the common includes.
