use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;

fn setup_test_files(test_name: &str) -> Result<(String, String, String), Box<dyn std::error::Error>> {
    // Create test directory if it doesn't exist
    fs::create_dir_all("test_files")?;
    
    let input = format!("test_files/{}_input.txt", test_name);
    let compressed = format!("test_files/{}_output.cmp", test_name);
    let decompressed = format!("test_files/{}_output.txt", test_name);
    
    Ok((input, compressed, decompressed))
}

fn cleanup_test_files(test_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input = format!("test_files/{}_input.txt", test_name);
    let compressed = format!("test_files/{}_output.cmp", test_name);
    let decompressed = format!("test_files/{}_output.txt", test_name);
    
    // Remove test files
    if Path::new(&input).exists() {
        fs::remove_file(&input)?;
    }
    if Path::new(&compressed).exists() {
        fs::remove_file(&compressed)?;
    }
    if Path::new(&decompressed).exists() {
        fs::remove_file(&decompressed)?;
    }
    
    // Remove test directory if empty
    if Path::new("test_files").exists() && fs::read_dir("test_files")?.next().is_none() {
        fs::remove_dir("test_files")?;
    }
    
    Ok(())
}

#[test]
fn test_compress_decompress_rle() -> Result<(), Box<dyn std::error::Error>> {
    let test_name = "rle";
    let (input, compressed, decompressed) = setup_test_files(test_name)?;

    // Create test file
    fs::write(&input, "AAABBBCCCCCDDDDE")?;

    // Compress
    let mut cmd = Command::cargo_bin("compressor")?;
    cmd.arg("compress")
        .arg(&input)
        .arg(&compressed)
        .arg("--rle")
        .assert()
        .success();

    // Decompress
    let mut cmd = Command::cargo_bin("compressor")?;
    cmd.arg("decompress")
        .arg(&compressed)
        .arg(&decompressed)
        .arg("--rle")
        .assert()
        .success();

    // Verify
    let original = fs::read(&input)?;
    let result = fs::read(&decompressed)?;
    assert_eq!(original, result);

    cleanup_test_files(test_name)?;
    Ok(())
}

#[test]
fn test_compress_decompress_lz() -> Result<(), Box<dyn std::error::Error>> {
    let test_name = "lz";
    let (input, compressed, decompressed) = setup_test_files(test_name)?;

    // Create test file
    fs::write(&input, "ABABABABABAB")?;

    // Compress
    let mut cmd = Command::cargo_bin("compressor")?;
    cmd.arg("compress")
        .arg(&input)
        .arg(&compressed)
        .arg("--lz")
        .assert()
        .success();

    // Decompress
    let mut cmd = Command::cargo_bin("compressor")?;
    cmd.arg("decompress")
        .arg(&compressed)
        .arg(&decompressed)
        .arg("--lz")
        .assert()
        .success();

    // Verify
    let original = fs::read(&input)?;
    let result = fs::read(&decompressed)?;
    assert_eq!(original, result);

    cleanup_test_files(test_name)?;
    Ok(())
}

#[test]
fn test_missing_algorithm() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("compressor")?;
    cmd.arg("compress")
        .arg("input.txt")
        .arg("output.cmp")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));

    Ok(())
}

#[test]
fn test_invalid_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("compressor")?;
    cmd.arg("compress")
        .arg("nonexistent.txt")
        .arg("output.cmp")
        .arg("--rle")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to open input file"));

    Ok(())
}
