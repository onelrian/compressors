/**
 * Run-Length Encoding (RLE) compression implementation
 * 
 * This module provides functions for compressing and decompressing data using
 * Run-Length Encoding. RLE is a simple compression algorithm that works well
 * for data with many repeated bytes.
 * 
 * @module rle
 */

/**
 * Maximum run length that can be encoded in a single pair
 * @constant {number}
 */
const MAX_RUN_LENGTH = 255;

/**
 * Compresses data using Run-Length Encoding (RLE)
 * 
 * @param {Buffer} data - Input data to compress
 * @returns {Buffer} Compressed data
 * @throws {Error} If input is not a Buffer
 */
function compress(data) {
    if (!Buffer.isBuffer(data)) {
        throw new Error('Input must be a Buffer');
    }

    if (!data || data.length === 0) {
        return Buffer.alloc(0);
    }

    const result = [];
    let currentByte = data[0];
    let count = 1;

    for (let i = 1; i < data.length; i++) {
        if (data[i] === currentByte && count < MAX_RUN_LENGTH) {
            count++;
        } else {
            result.push(count, currentByte);
            currentByte = data[i];
            count = 1;
        }
    }
    result.push(count, currentByte);

    return Buffer.from(result);
}

/**
 * Decompresses RLE-encoded data
 * 
 * @param {Buffer} data - Compressed data
 * @returns {Buffer} Decompressed data
 * @throws {Error} If input is not a Buffer or has invalid format
 */
function decompress(data) {
    if (!Buffer.isBuffer(data)) {
        throw new Error('Input must be a Buffer');
    }

    if (!data || data.length === 0) {
        return Buffer.alloc(0);
    }

    if (data.length % 2 !== 0) {
        throw new Error('Invalid RLE data format: expected pairs of (count, byte)');
    }

    const result = [];
    for (let i = 0; i < data.length; i += 2) {
        const count = data[i];
        const byte = data[i + 1];
        
        if (count < 1 || count > MAX_RUN_LENGTH) {
            throw new Error(`Invalid run length: ${count} (must be between 1 and ${MAX_RUN_LENGTH})`);
        }

        for (let j = 0; j < count; j++) {
            result.push(byte);
        }
    }

    return Buffer.from(result);
}

module.exports = {
    compress,
    decompress
};