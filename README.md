# rustlink

[![Codespell CI](https://github.com/gemesa/rustlink/actions/workflows/codespell.yml/badge.svg?event=push)](https://github.com/gemesa/rustlink/actions/workflows/codespell.yml)
[![Markdown link CI](https://github.com/gemesa/rustlink/actions/workflows/md-link.yml/badge.svg?event=push)](https://github.com/gemesa/rustlink/actions/workflows/md-link.yml)

:warning: Work in progress! :warning:

rustlink is a set of tools to program STM32 devices:
- `st-info`: chip and device information tool
- `st-flash`: programmer and flash manipulation tool

## Installation from source

### Prerequisites

The following tools are necessary for building:

- `cargo` (>=1.68.2)
- `rustc` (>=1.68.2)

Both can be installed from the [official site](https://www.rust-lang.org/tools/install).

### How to build and install

Invoke the following commands:

```bash
$ cargo build --release
$ cargo install --path .
```

which will build `st-info` and `st-flash` executables and install them in `<your-home>/.cargo/bin/`.
