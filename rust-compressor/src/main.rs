//! A command-line compression tool supporting RLE and LZ77 algorithms
//!
//! This tool provides file compression and decompression using two algorithms:
//! - Run-Length Encoding (RLE)
//! - Simplified LZ77
//!
//! # Usage
//! ```bash
//! compress|decompress <input_file> <output_file> --rle|--lz
//! ```
//! Use '-' for stdin/stdout

use anyhow::{Context, Result};
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;

mod lz;
mod rle;

/// Main entry point for the compression tool
///
/// # Arguments
/// The program expects the following arguments:
/// 1. Operation: "compress" or "decompress"
/// 2. Input file path (or "-" for stdin)
/// 3. Output file path (or "-" for stdout)
/// 4. Algorithm: "--rle" or "--lz"
///
/// # Errors
/// Returns an error if:
/// - Invalid number of arguments
/// - Invalid operation or algorithm
/// - File operations fail
/// - Compression/decompression fails
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        eprintln!(
            "Usage: {} [compress|decompress] [input_file|-] [output_file|-] [--rle|--lz]",
            args[0]
        );
        eprintln!("Example: {} compress input.txt output.txt.cmp --rle", args[0]);
        process::exit(1);
    }

    let operation = &args[1];
    let input_path = &args[2];
    let output_path = &args[3];
    let algorithm = &args[4];

    // Determine input source
    let mut input: Box<dyn Read> = if input_path == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(input_path).with_context(|| format!("Failed to open input file: {}", input_path))?)
    };

    // Determine output destination
    let mut output: Box<dyn Write> = if output_path == "-" {
        Box::new(io::stdout())
    } else {
        Box::new(File::create(output_path).with_context(|| format!("Failed to create output file: {}", output_path))?)
    };

    // Process data based on operation and algorithm
    match (operation.as_str(), algorithm.as_str()) {
        ("compress", "--rle") => rle::compress(&mut input, &mut output)
            .with_context(|| "RLE compression failed")?,
        ("decompress", "--rle") => rle::decompress(&mut input, &mut output)
            .with_context(|| "RLE decompression failed")?,
        ("compress", "--lz") => lz::compress(&mut input, &mut output)
            .with_context(|| "LZ77 compression failed")?,
        ("decompress", "--lz") => lz::decompress(&mut input, &mut output)
            .with_context(|| "LZ77 decompression failed")?,
        _ => {
            eprintln!("Please specify compression algorithm (--rle or --lz)");
            process::exit(1);
        }
    }

    output.flush().with_context(|| "Failed to flush output")?;
    Ok(())
}
