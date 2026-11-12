# Rusty Printer

A lightweight printing utility written in Rust for interacting with system printers and managing print jobs.

## Features

- List available printers
- Use the default system printer
- Print documents directly from the command line
- Simple and lightweight Rust implementation
- Linux printing support through CUPS

## Installation

```bash
git clone https://github.com/k1toune/rusty-printer.git
cd rusty-printer
cargo build --release
```

## Usage

### List printers

```bash
cargo run -- list
```

### Print a document

```bash
cargo run -- print document.pdf
```

### Build

```bash
cargo build --release
```

## Requirements

- Rust
- Cargo
- CUPS (Linux)

## License

MIT
