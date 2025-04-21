const assert = require('assert');
const rle = require('../rle');

describe('RLE Compression', () => {
    it('should compress and decompress empty data', () => {
        const data = Buffer.alloc(0);
        const compressed = rle.compress(data);
        const decompressed = rle.decompress(compressed);
        assert.deepStrictEqual(decompressed, data);
    });

    it('should compress and decompress single byte', () => {
        const data = Buffer.from('A');
        const compressed = rle.compress(data);
        const decompressed = rle.decompress(compressed);
        assert.deepStrictEqual(decompressed, data);
    });

    it('should compress and decompress repeated bytes', () => {
        const data = Buffer.from('AAAABBBCCCC');
        const compressed = rle.compress(data);
        const decompressed = rle.decompress(compressed);
        assert.deepStrictEqual(decompressed, data);
    });

    it('should handle maximum run length', () => {
        const data = Buffer.from('A'.repeat(255));
        const compressed = rle.compress(data);
        const decompressed = rle.decompress(compressed);
        assert.deepStrictEqual(decompressed, data);
    });

    it('should throw error for invalid input', () => {
        assert.throws(() => rle.compress('not a buffer'));
        assert.throws(() => rle.decompress('not a buffer'));
    });

    it('should throw error for invalid format', () => {
        const invalidData = Buffer.from([1, 2, 3]); // Not a multiple of 2
        assert.throws(() => rle.decompress(invalidData));
    });
}); 