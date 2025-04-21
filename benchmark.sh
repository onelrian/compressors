#!/bin/bash

# Create test files
echo "Creating test files..."
mkdir -p test_files

# Generate sample files of different sizes
dd if=/dev/urandom of=test_files/random_1M.bin bs=1M count=1
dd if=/dev/zero of=test_files/zeros_1M.bin bs=1M count=1
yes "ABC" | head -c 1M > test_files/repeating_1M.bin
cat /usr/share/dict/words | head -c 1M > test_files/text_1M.bin

# Function to run benchmark
run_benchmark() {
    local impl=$1
    local algo=$2
    local file=$3

    echo -e "\nBenchmarking $impl with $algo on $(basename $file)"
    
    # Measure Compression Time
    echo "Compressing..."
    compression_time=$( (time docker run -v $(pwd):/data ghcr.io/onelrian/compressors/$impl-compressor compress /data/$file /data/${file}.cmp --$algo) 2>&1 | grep real | awk '{print $2}' )

    # Measure Decompression Time
    echo "Decompressing..."
    decompression_time=$( (time docker run -v $(pwd):/data ghcr.io/onelrian/compressors/$impl-compressor decompress /data/${file}.cmp /data/${file}.dec --$algo) 2>&1 | grep real | awk '{print $2}' )

    # Get compressed size and original size
    compressed_size=$(stat -c%s "${file}.cmp")
    original_size=$(stat -c%s "$file")

    # Calculate compression ratio
    ratio=$(echo "scale=2; $compressed_size / $original_size * 100" | bc)

    # Output the results
    echo "Compression time: $compression_time"
    echo "Decompression time: $decompression_time"
    echo "Compression ratio: ${ratio}%"

    # Verify decompression
    if cmp -s "$file" "${file}.dec"; then
        echo "Verification: SUCCESS"
    else
        echo "Verification: FAILED"
    fi
    
    # Clean up
    rm "${file}.cmp" "${file}.dec"
}

# Run benchmarks on different files, algorithms, and implementations
for file in test_files/*.bin; do
    for impl in rust js; do
        for algo in rle lz; do
            run_benchmark $impl $algo $file
        done
    done
done

# Cleanup test files
rm -rf test_files
