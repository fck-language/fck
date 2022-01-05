# Installing

This document outlines the installation of fck for both installing pre-built binaries and building from sauce; as well as configuration.

## Contents

## Important note

To run fck, you need to have a config file at `$HOME/.fck`. Since fck is centered around multilingual coding, we can't assume what language you're using and need you to specify. This is laced on the first line of the config file in the form of a language code from ISO 639-1, such as `en` for English.

## Pre-Built Binaries

This is by far the easiest and fastest way to install fck. With each release, pre-built binaries for all flavours are provided for Linux (main distros), MacOS (Intel and M1 chip) and Windows.
the table below shows where to place this executable:

| OS      | Location         |
|---------|------------------|
| Linux   | `/`              |
| MacOS   | `/usr/local/bin` |
| Windows | `/`              |

## Building From Sauce

> This is much more involved than just downloading a pre-built binary. It also takes substantially more time. You've been warned.

fck relies upon LLVM, so to build from sauce you need to download and build LLVM. We are currently on version 13.0.0. You can find this at the [LLVM repo](https://github.com/llvm/llvm-project). Download v13.0.0, unzip the file and build with 
```shell
cmake --build . --target install
```
from inside the directory. This does take some time so go make yourself a snack.

Once build, you can build fck using
```shell
LLVM_SYS_130_PREFIX=path/to/llvm-13.0.0 cargo build --release --bin fck_pure --package fck_pure
```

Once built, move the executable to anywhere included in your `$PATH`

**UNTESTED** You can alternatively use `cargo install` instead of `cargo run`

## Configuration

A config file is placed in `$HOME` and is in the file `.fck` The first line must be a language code, or fck will not run. The rest of this section is a table with all the options available in the config file:

| Key              | Default | Description                                                            |
|------------------|---------|------------------------------------------------------------------------|
| `wraptLength`    | 70      | Default length of information printed                                  |
| `defaultCompile` | true    | Determines if fck defaults to compile or interpret mode by default     |
| `historyLength`  | 100     | Length of the history of the shell (number of previous commands saved) |
