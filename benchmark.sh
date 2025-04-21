#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Create test files
echo "Creating test files..."
echo "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA" > test1.txt
echo "ABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABAB" > test2.txt
dd if=/dev/urandom of=test3.txt bs=1M count=1 2>/dev/null

# Function to run benchmark
run_benchmark() {
    local impl=$1
    local algo=$2
    local test_file=$3
    local compressed_file="${test_file}.cmp"
    local decompressed_file="${test_file}.dec"

    echo -e "\n${YELLOW}Testing $impl with $algo on $test_file${NC}"

    # Compression
    echo "Compressing..."
    time docker run --rm -v "$(pwd):/data" ghcr.io/$(basename $(pwd))/$impl compress "/data/$test_file" "/data/$compressed_file" --$algo

    # Get compressed size
    compressed_size=$(stat -f %z "$compressed_file" 2>/dev/null || stat -c %s "$compressed_file")
    original_size=$(stat -f %z "$test_file" 2>/dev/null || stat -c %s "$test_file")
    ratio=$(echo "scale=2; $compressed_size * 100 / $original_size" | bc)
    echo -e "${GREEN}Compression ratio: ${ratio}%${NC}"

    # Decompression
    echo "Decompressing..."
    time docker run --rm -v "$(pwd):/data" ghcr.io/$(basename $(pwd))/$impl decompress "/data/$compressed_file" "/data/$decompressed_file" --$algo

    # Verify
    if cmp -s "$test_file" "$decompressed_file"; then
        echo -e "${GREEN}Verification successful${NC}"
    else
        echo -e "${RED}Verification failed${NC}"
    fi

    # Cleanup
    rm -f "$compressed_file" "$decompressed_file"
}

# Run benchmarks
echo -e "\n${YELLOW}Starting benchmarks...${NC}"

for test_file in test1.txt test2.txt test3.txt; do
    echo -e "\n${YELLOW}Testing file: $test_file${NC}"
    
    # Test RLE
    run_benchmark "rust-compressor" "rle" "$test_file"
    run_benchmark "js-compressor" "rle" "$test_file"
    
    # Test LZ77
    run_benchmark "rust-compressor" "lz" "$test_file"
    run_benchmark "js-compressor" "lz" "$test_file"
done

# Cleanup test files
echo -e "\nCleaning up..."
rm -f test1.txt test2.txt test3.txt

echo -e "\n${GREEN}Benchmark completed${NC}"
