Got it! Here's an updated, more concise version of the `README.md` with the specific instructions to `chmod` the `benchmark.sh` script included:

---

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

## Benchmarking

To benchmark the compression and decompression performance:

1. **Make the `benchmark.sh` script executable:**

   ```bash
   chmod +x benchmark.sh
   ```

2. **Run the benchmark:**

   ```bash
   ./benchmark.sh
   ```

The script will compare compression time, decompression time, and compression ratio for both implementations (Rust and JavaScript) using sample files.

## License
MIT License.
