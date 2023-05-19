# rustlink

[![Codespell CI](https://github.com/gemesa/rustlink/actions/workflows/codespell.yml/badge.svg?event=push)](https://github.com/gemesa/rustlink/actions/workflows/codespell.yml)
[![Markdown link CI](https://github.com/gemesa/rustlink/actions/workflows/md-link.yml/badge.svg?event=push)](https://github.com/gemesa/rustlink/actions/workflows/md-link.yml)
[![Rust CI](https://github.com/gemesa/rustlink/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/gemesa/rustlink/actions/workflows/rust.yml)
[![Shellcheck CI](https://github.com/gemesa/rustlink/actions/workflows/shellcheck.yml/badge.svg?event=push)](https://github.com/gemesa/rustlink/actions/workflows/shellcheck.yml)

rustlink is a set of tools to program STM32 devices:
- `rst-info`: chip and device information tool
- `rst-flash`: programmer and flash manipulation tool

<!-- markdown-link-check-disable-next-line -->
The motivation behind these tools is that [probe-rs-cli](https://crates.io/crates/probe-rs-cli) can not handle multiple connected probes (devices):

```
$ probe-rs-cli info
Error: 2 probes were found.
```

With `rst-flash` you can choose by serial number which device to use:

```
$ rst-info list
[0]: STLink V2 - serial: XXXXXXXXXXXXXXXXXXXXXXXXXXXX
[1]: STLink V2 - serial: YYYYYYYYYYYYYYYYYYYYYYYYYYYY
$ rst-flash download YYYYYYYYYYYYYYYYYYYYYYYYYYYY STM32F103C8 app.elf
     Erasing sectors ✔ [00:00:00] [########################################################################################################] 1.00 KiB/1.00 KiB @ 16.57 KiB/s (eta 0s )
 Programming pages   ✔ [00:00:00] [#########################################################################################################] 1.00 KiB/1.00 KiB @ 9.87 KiB/s (eta 0s )
    Finished in 0.209s      
```

## Installation from source

### Prerequisites

The following tools are necessary for building:

- `cargo` (>=1.68.2)
- `rustc` (>=1.68.2)

Both can be installed from the [official Rust site](https://www.rust-lang.org/tools/install).

### How to build and install

Invoke the following commands:

```bash
$ cargo build --release
$ cargo install --path .
```

which will build `rst-info` and `rst-flash` executables and install them in `<your-home>/.cargo/bin/`.
