# rustlink

[![Codespell CI](https://github.com/gemesa/rustlink/actions/workflows/codespell.yml/badge.svg?event=push)](https://github.com/gemesa/rustlink/actions/workflows/codespell.yml)
[![Markdown link CI](https://github.com/gemesa/rustlink/actions/workflows/md-link.yml/badge.svg?event=push)](https://github.com/gemesa/rustlink/actions/workflows/md-link.yml)
[![Rust CI](https://github.com/gemesa/rustlink/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/gemesa/rustlink/actions/workflows/rust.yml)
[![Shellcheck CI](https://github.com/gemesa/rustlink/actions/workflows/shellcheck.yml/badge.svg?event=push)](https://github.com/gemesa/rustlink/actions/workflows/shellcheck.yml)

:warning: Work in progress! :warning:

rustlink is a set of tools to program STM32 devices:
- `rst-info`: chip and device information tool
- `rst-flash`: programmer and flash manipulation tool

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

which will build `rst-info` and `rst-flash` executables and install them in `<your-home>/.cargo/bin/`.
