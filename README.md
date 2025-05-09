
# fget

A Rust-powered download manager built for speed, flexibility, and integration with modern workflows.

## Installation

1. Download a pre-built binary from the [Releases](https://github.com/theunrealtark/fget/releases) page, or compile it yourself (see below).
2. Move the binary to a directory in your `$PATH`, such as `/usr/local/bin` or `/usr/bin`.

## Usage

To see available options and flags, run:

```bash
fget --help
```

Example:

```bash
fget --url https://ash-speed.hetzner.com/1GB.bin
```

## Development

To build from source:

```bash
git clone https://github.com/theunrealtark/fget
cd fget
sh build.sh
```

Make sure you have Rust installed via [rustup](https://rustup.rs).

## Features

* CLI-based download manager
* Chrome extension integration (via `--listen` mode)
* Smart filename resolution
* File conflict handling with user prompts
* Clean progress bar UI with `cliclack`

## Links

* [GitHub Repository](https://github.com/theunrealtark/fget)
* [Issue Tracker](https://github.com/theunrealtark/fget/issues)

