# rat

A reimplementation of `cat` in Rust.

# Motivation

Just for fun, it's a simple program to implement as a learning project.

# Features

It can do (almost) anything the Unix/Linux version of `cat` can do.
Currently it is lacking support for the `-v`/`--show-nonprinting` option as well
as suport for reading from stdin.

# Installation

**Requirements:** As it is written in rust and there are no pre-built binaries
`rustc` as well as `cargo` have to be installed on your system.

1. Run `cargo build --release` in this directory
2. Find the executable binary at `target/release/rat(.exe)`
3. Copy the executable somewhere you want to keep it and add it to path

# Usage

The options are the same as the Unix/Linux version of `cat`.
`rat -h` will display help information.

Example:
```
$ rat -ns ./poem.txt
```

# License

Licensed under the [MIT License](LICENSE.md)
