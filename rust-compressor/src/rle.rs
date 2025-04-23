//! Run-Length Encoding (RLE) compression implementation
//!
//! This module provides functions for compressing and decompressing data using
//! Run-Length Encoding. RLE is a simple compression algorithm that works well
//! for data with many repeated bytes.
//!
//! # Format
//! The compressed data is stored as pairs of (count, byte) where:
//! - count: u8 (1-255) representing the number of repetitions
//! - byte: the repeated byte value

use std::io::{Read, Write};
use anyhow::{Context, Result};

/// Maximum run length that can be encoded in a single pair
const MAX_RUN_LENGTH: u8 = 255;

/// Compresses data using Run-Length Encoding (RLE)
///
/// # Arguments
/// * `input` - A reader implementing the Read trait
/// * `output` - A writer implementing the Write trait
///
/// # Returns
/// Returns `Ok(())` on success, or an error if compression fails
///
/// # Errors
/// Returns an error if:
/// - Reading from input fails
/// - Writing to output fails
pub fn compress<R: Read, W: Write>(input: &mut R, output: &mut W) -> Result<()> {
    eprintln!("Starting RLE compression");
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)
        .with_context(|| "Failed to read input data")?;
    eprintln!("Read {} bytes from input", buffer.len());

    if buffer.is_empty() {
        eprintln!("Input is empty, nothing to compress");
        return Ok(());
    }

    let mut current_byte = buffer[0];
    let mut count = 1;

    for &byte in buffer.iter().skip(1) {
        if byte == current_byte && count < MAX_RUN_LENGTH {
            count += 1;
        } else {
            output.write_all(&[count, current_byte])
                .with_context(|| "Failed to write compressed data")?;
            current_byte = byte;
            count = 1;
        }
    }
    output.write_all(&[count, current_byte])
        .with_context(|| "Failed to write final compressed data")?;
    eprintln!("Compression complete");

    Ok(())
}

/// Decompresses RLE-encoded data
///
/// # Arguments
/// * `input` - A reader implementing the Read trait
/// * `output` - A writer implementing the Write trait
///
/// # Returns
/// Returns `Ok(())` on success, or an error if decompression fails
///
/// # Errors
/// Returns an error if:
/// - Reading from input fails
/// - Writing to output fails
/// - Input data is not properly formatted (not pairs of count and byte)
pub fn decompress<R: Read, W: Write>(input: &mut R, output: &mut W) -> Result<()> {
    eprintln!("Starting RLE decompression");
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)
        .with_context(|| "Failed to read compressed data")?;
    eprintln!("Read {} bytes from input", buffer.len());

    if buffer.is_empty() {
        eprintln!("Input is empty, nothing to decompress");
        return Ok(());
    }

    for chunk in buffer.chunks(2) {
        if chunk.len() != 2 {
            return Err(anyhow::anyhow!("Invalid RLE data format: expected pairs of (count, byte)"));
        }
        let count = chunk[0];
        let byte = chunk[1];
        output.write_all(&vec![byte; count as usize])
            .with_context(|| format!("Failed to write decompressed data for byte {}", byte))?;
    }
    eprintln!("Decompression complete");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_rle_roundtrip() {
        let input = b"AAABBBCCCCCDDDDE";
        let mut compressed = Vec::new();
        compress(&mut Cursor::new(input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(&mut Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed.as_slice());
    }

    #[test]
    fn test_rle_empty() {
        let input = b"";
        let mut compressed = Vec::new();
        compress(&mut Cursor::new(input), &mut compressed).unwrap();
        assert!(compressed.is_empty());

        let mut decompressed = Vec::new();
        decompress(&mut Cursor::new(compressed), &mut decompressed).unwrap();
        assert!(decompressed.is_empty());
    }

    #[test]
    fn test_rle_single_byte() {
        let input = b"A";
        let mut compressed = Vec::new();
        compress(&mut Cursor::new(input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(&mut Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed.as_slice());
    }

    #[test]
    fn test_rle_max_run() {
        let input = vec![b'A'; 255];
        let mut compressed = Vec::new();
        compress(&mut Cursor::new(&input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(&mut Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed);
    }

    #[test]
    fn test_rle_invalid_format() {
        let invalid_data = vec![1, 2, 3]; // Not a multiple of 2
        let mut output = Vec::new();
        let result = decompress(&mut Cursor::new(invalid_data), &mut output);
        assert!(result.is_err());
    }
}
