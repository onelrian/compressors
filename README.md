# Compressor Tool

A tool that implements **RLE** and **LZ77** compression algorithms in both **Rust** and **JavaScript**.

## Features

- **Compress** and **Decompress** using RLE or LZ77.

## Prerequisites

### Rust
- Install Rust & Cargo: [Install Rust](https://www.rust-lang.org/learn/get-started)

### JavaScript
- Install Node.js & npm: [Install Node.js](https://nodejs.org/)

## Setup

### Rust
1. Clone the repo:
   ```bash
   git clone https://github.com/your-repo/compressor-tool.git
   cd compressor-tool/rust
   cargo build --release
   ```

### JavaScript
1. Clone the repo:
   ```bash
   git clone https://github.com/your-repo/compressor-tool.git
   cd compressor-tool/js
   npm install
   ```

## Usage

### Rust
```bash
cargo run -- [compress|decompress] [--rle|--lz] [input_file] [output_file]
```

### JavaScript
```bash
npm start [compress|decompress] [--rle|--lz] [input_file] [output_file]
```

## License
MIT License.