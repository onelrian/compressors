//! Simplified LZ77 compression implementation
//!
//! This module provides functions for compressing and decompressing data using
//! a simplified version of the LZ77 algorithm. The implementation uses a fixed
//! sliding window and encodes matches and literals in a simple format.
//!
//! # Format
//! The compressed data consists of two types of tokens:
//! - Literal: `0x00 + byte` - Represents a single byte
//! - Match: `0x01 + offset + length` - Represents a repeated sequence
//!
//! # Parameters
//! - Window size: 20 bytes
//! - Maximum match length: 255 bytes

use std::io::{Read, Write};
use anyhow::{Context, Result};

/// Size of the sliding window for LZ77 compression
const WINDOW_SIZE: usize = 20;

/// Maximum length of a match that can be encoded
const MAX_MATCH_LENGTH: u8 = 255;

/// Compresses data using simplified LZ77 algorithm
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
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)
        .with_context(|| "Failed to read input data")?;

    if buffer.is_empty() {
        return Ok(());
    }

    let mut pos = 0;
    while pos < buffer.len() {
        let mut best_match = (0, 0);
        let search_start = pos.saturating_sub(WINDOW_SIZE);
        let search_end = pos;

        // Find the longest match in the sliding window
        for offset in search_start..search_end {
            let mut length = 0;
            while pos + length < buffer.len()
                && buffer[offset + length] == buffer[pos + length]
                && length < MAX_MATCH_LENGTH as usize
            {
                length += 1;
            }
            if length > best_match.1 {
                best_match = (pos - offset, length);
            }
        }

        if best_match.1 > 2 {
            // Encode as a match
            output.write_all(&[0x01, best_match.0 as u8, best_match.1 as u8])
                .with_context(|| "Failed to write match token")?;
            pos += best_match.1;
        } else {
            // Encode as a literal
            output.write_all(&[0x00, buffer[pos]])
                .with_context(|| "Failed to write literal token")?;
            pos += 1;
        }
    }

    Ok(())
}

/// Decompresses LZ77-encoded data
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
/// - Input data is not properly formatted
/// - Invalid token type encountered
/// - Invalid offset or length in match token
pub fn decompress<R: Read, W: Write>(input: &mut R, output: &mut W) -> Result<()> {
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)
        .with_context(|| "Failed to read compressed data")?;

    if buffer.is_empty() {
        return Ok(());
    }

    let mut result = Vec::new();
    let mut pos = 0;

    while pos < buffer.len() {
        let token_type = buffer[pos];
        pos += 1;

        match token_type {
            0x00 => {
                // Literal
                if pos >= buffer.len() {
                    return Err(anyhow::anyhow!("Invalid LZ77 data format: missing literal byte"));
                }
                result.push(buffer[pos]);
                pos += 1;
            }
            0x01 => {
                // Match
                if pos + 1 >= buffer.len() {
                    return Err(anyhow::anyhow!("Invalid LZ77 data format: incomplete match token"));
                }
                let offset = buffer[pos] as usize;
                let length = buffer[pos + 1] as usize;
                pos += 2;

                if offset > result.len() {
                    return Err(anyhow::anyhow!(
                        "Invalid offset in LZ77 data: offset {} exceeds current output length {}",
                        offset,
                        result.len()
                    ));
                }

                let start = result.len() - offset;
                for i in 0..length {
                    result.push(result[start + i]);
                }
            }
            _ => return Err(anyhow::anyhow!(
                "Invalid token type in LZ77 data: expected 0x00 or 0x01, got 0x{:02x}",
                token_type
            )),
        }
    }

    output.write_all(&result)
        .with_context(|| "Failed to write decompressed data")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_lz_roundtrip() {
        let input = b"ABABABABABAB";
        let mut compressed = Vec::new();
        compress(&mut Cursor::new(input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(&mut Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed.as_slice());
    }

    #[test]
    fn test_lz_empty() {
        let input = b"";
        let mut compressed = Vec::new();
        compress(&mut Cursor::new(input), &mut compressed).unwrap();
        assert!(compressed.is_empty());

        let mut decompressed = Vec::new();
        decompress(&mut Cursor::new(compressed), &mut decompressed).unwrap();
        assert!(decompressed.is_empty());
    }

    #[test]
    fn test_lz_single_byte() {
        let input = b"A";
        let mut compressed = Vec::new();
        compress(&mut Cursor::new(input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(&mut Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed.as_slice());
    }

    #[test]
    fn test_lz_non_repeating() {
        let input = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut compressed = Vec::new();
        compress(&mut Cursor::new(input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(&mut Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed.as_slice());
    }

    #[test]
    fn test_lz_mixed_data() {
        let input = b"ABCABCABCXYZXYZXYZ123123123";
        let mut compressed = Vec::new();
        compress(&mut Cursor::new(input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(&mut Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed.as_slice());
    }

    #[test]
    fn test_lz_invalid_token() {
        let invalid_data = vec![0x02]; // Invalid token type
        let mut output = Vec::new();
        let result = decompress(&mut Cursor::new(invalid_data), &mut output);
        assert!(result.is_err());
    }

    #[test]
    fn test_lz_incomplete_match() {
        let invalid_data = vec![0x01, 0x01]; // Missing length
        let mut output = Vec::new();
        let result = decompress(&mut Cursor::new(invalid_data), &mut output);
        assert!(result.is_err());
    }

    #[test]
    fn test_lz_invalid_offset() {
        let invalid_data = vec![0x01, 0x05, 0x01]; // Offset 5 with empty output
        let mut output = Vec::new();
        let result = decompress(&mut Cursor::new(invalid_data), &mut output);
        assert!(result.is_err());
    }
}
