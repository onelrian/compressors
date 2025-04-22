# Compression Tool

A command-line compression tool implementing Run-Length Encoding (RLE) and Simplified LZ77 algorithms in both Rust and JavaScript.

## Features

- Two compression algorithms:
  - Run-Length Encoding (RLE)
  - Simplified LZ77
- Implementations in both Rust and JavaScript
- Docker support for easy deployment
- Comprehensive test coverage
- Performance benchmarking

## Installation

### Docker (Recommended)

```bash
# Pull the Docker images
docker pull ghcr.io/onelrian/rust-compressor
docker pull ghcr.io/your-org-name/js-compressor
```

### From Source

#### Rust Implementation

```bash
cd rust-compressor
cargo build --release
```

#### JavaScript Implementation

```bash
cd js-compressor
npm install
```

## Usage

### Basic Usage

```bash
# Compress a file using RLE
compress <input_file> <output_file> --rle

# Decompress a file using RLE
decompress <input_file> <output_file> --rle

# Compress a file using LZ77
compress <input_file> <output_file> --lz

# Decompress a file using LZ77
decompress <input_file> <output_file> --lz
```

### Using Docker

```bash
# Compress a file using RLE (Rust implementation)
docker run -v $(pwd):/data ghcr.io/onelrian/rust-compressor compress /data/input.txt /data/output.txt.cmp --rle

# Decompress a file using LZ77 (JavaScript implementation)
docker run -v $(pwd):/data ghcr.io/onelrian/js-compressor decompress /data/input.txt.cmp /data/output.txt --lz
```

## Algorithms

### Run-Length Encoding (RLE)

RLE is a simple compression algorithm that works well for data with many repeated bytes. It stores repeated bytes as a pair of (count, byte).

Example:
```
Input:  "AAAABBBCCCC"
Output: [(4, 'A'), (3, 'B'), (4, 'C')]
```

### Simplified LZ77

LZ77 is a dictionary-based compression algorithm that works by finding repeated sequences in the input data. Our implementation uses a fixed sliding window of 20 bytes.

The compressed data consists of two types of tokens:
- Literal: `0x00 + byte` - Represents a single byte
- Match: `0x01 + offset + length` - Represents a repeated sequence

## Development

### Building

```bash
# Build Rust implementation
cd rust-compressor
cargo build

# Build JavaScript implementation
cd js-compressor
npm install
```

### Testing

```bash
# Test Rust implementation
cd rust-compressor
cargo test

# Test JavaScript implementation
cd js-compressor
npm test
```

### Benchmarking

```bash
# Run benchmarks
./benchmark.sh
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
