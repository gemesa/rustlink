# rustlink

[![Codespell CI](https://github.com/gemesa/rustlink/actions/workflows/codespell.yml/badge.svg?event=push)](https://github.com/gemesa/rustlink/actions/workflows/codespell.yml)
[![Markdown link CI](https://github.com/gemesa/rustlink/actions/workflows/md-link.yml/badge.svg?event=push)](https://github.com/gemesa/rustlink/actions/workflows/md-link.yml)
[![Rust CI](https://github.com/gemesa/rustlink/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/gemesa/rustlink/actions/workflows/rust.yml)
[![Shellcheck CI](https://github.com/gemesa/rustlink/actions/workflows/shellcheck.yml/badge.svg?event=push)](https://github.com/gemesa/rustlink/actions/workflows/shellcheck.yml)

rustlink is a set of tools to program STM32 devices:
- `rst-info`: device information tool
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
[0]: STLink V2 - serial: AAAAAAAAAAAAAAAAAAAAAAAAAAAA
[1]: STLink V2 - serial: BBBBBBBBBBBBBBBBBBBBBBBBBBBB
$ rst-flash download -s AAAAAAAAAAAAAAAAAAAAAAAAAAAA -t STM32F103C8 -f app.elf
    Finished in 0.179s
$ rst-flash reset -s AAAAAAAAAAAAAAAAAAAAAAAAAAAA -t STM32F103C8
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

## Examples

### List STlink devices

```
$ rst-info list
[0]: STLink V2 - serial: AAAAAAAAAAAAAAAAAAAAAAAAAAAA
[1]: STLink V2 - serial: BBBBBBBBBBBBBBBBBBBBBBBBBBBB
```

### Download to flash and run

```
$ rst-flash download -s AAAAAAAAAAAAAAAAAAAAAAAAAAAA -t STM32F103C8 -f app.elf
    Finished in 0.179s
$ rst-flash reset -s AAAAAAAAAAAAAAAAAAAAAAAAAAAA -t STM32F103C8
```

### Dump and erase memory

```
$ rst-flash download -s AAAAAAAAAAAAAAAAAAAAAAAAAAAA -t STM32F103C8 -f app.elf 
    Finished in 0.191s
$ rst-flash dump -s AAAAAAAAAAAAAAAAAAAAAAAAAAAA -t STM32F103C8 0x0800_0000 1                   
Addr 0x08000000: 0x20005000
Read 1 words in 498.057µs
$ rst-flash erase -s AAAAAAAAAAAAAAAAAAAAAAAAAAAA -t STM32F103C8
$ rst-flash dump -s AAAAAAAAAAAAAAAAAAAAAAAAAAAA -t STM32F103C8 0x0800_0000 1
Addr 0x08000000: 0xffffffff
Read 1 words in 473.943µs
```

### Get chip info

```
$ probe-rs-cli chip info STM32F103C8
STM32F103C8
Cores (1):
    - main (Armv7m)
RAM: 0x20000000..0x20005000 (20.00 KiB)
NVM: 0x08000000..0x08010000 (64.00 KiB)
```
