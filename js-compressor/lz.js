/**
 * Simplified LZ77 compression implementation
 * 
 * This module provides functions for compressing and decompressing data using
 * a simplified version of the LZ77 algorithm. The implementation uses a fixed
 * sliding window and encodes matches and literals in a simple format.
 * 
 * @module lz
 */

/**
 * Size of the sliding window for LZ77 compression
 * @constant {number}
 */
const WINDOW_SIZE = 20;

/**
 * Maximum length of a match that can be encoded
 * @constant {number}
 */
const MAX_MATCH_LENGTH = 255;

/**
 * Compresses data using simplified LZ77 algorithm
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
    let pos = 0;

    while (pos < data.length) {
        let bestMatch = { offset: 0, length: 0 };
        const searchStart = Math.max(0, pos - WINDOW_SIZE);
        const searchEnd = pos;

        // Find the longest match in the sliding window
        for (let offset = searchStart; offset < searchEnd; offset++) {
            let length = 0;
            while (pos + length < data.length && 
                   data[offset + length] === data[pos + length] && 
                   length < MAX_MATCH_LENGTH) {
                length++;
            }
            if (length > bestMatch.length) {
                bestMatch = { offset: pos - offset, length };
            }
        }

        if (bestMatch.length > 2) {
            // Encode as a match
            result.push(0x01, bestMatch.offset, bestMatch.length);
            pos += bestMatch.length;
        } else {
            // Encode as a literal
            result.push(0x00, data[pos]);
            pos++;
        }
    }

    return Buffer.from(result);
}

/**
 * Decompresses LZ77-encoded data
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

    const result = [];
    let pos = 0;

    while (pos < data.length) {
        const type = data[pos++];
        
        if (type === 0x00) {
            // Literal
            if (pos >= data.length) {
                throw new Error('Invalid LZ77 data format: missing literal byte');
            }
            result.push(data[pos++]);
        } else if (type === 0x01) {
            // Match
            if (pos + 1 >= data.length) {
                throw new Error('Invalid LZ77 data format: incomplete match token');
            }
            const offset = data[pos++];
            const length = data[pos++];
            
            if (offset > result.length) {
                throw new Error(`Invalid offset in LZ77 data: offset ${offset} exceeds current output length ${result.length}`);
            }

            const start = result.length - offset;
            for (let i = 0; i < length; i++) {
                result.push(result[start + i]);
            }
        } else {
            throw new Error(`Invalid token type in LZ77 data: expected 0x00 or 0x01, got 0x${type.toString(16).padStart(2, '0')}`);
        }
    }

    return Buffer.from(result);
}

module.exports = {
    compress,
    decompress
};