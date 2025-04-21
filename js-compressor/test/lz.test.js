const assert = require('assert');
const lz = require('../lz');

describe('LZ77 Compression', () => {
    it('should compress and decompress empty data', () => {
        const data = Buffer.alloc(0);
        const compressed = lz.compress(data);
        const decompressed = lz.decompress(compressed);
        assert.deepStrictEqual(decompressed, data);
    });

    it('should compress and decompress single byte', () => {
        const data = Buffer.from('A');
        const compressed = lz.compress(data);
        const decompressed = lz.decompress(compressed);
        assert.deepStrictEqual(decompressed, data);
    });

    it('should compress and decompress repeated pattern', () => {
        const data = Buffer.from('ABABABABABAB');
        const compressed = lz.compress(data);
        const decompressed = lz.decompress(compressed);
        assert.deepStrictEqual(decompressed, data);
    });

    it('should compress and decompress non-repeating data', () => {
        const data = Buffer.from('ABCDEFGHIJKLMNOPQRSTUVWXYZ');
        const compressed = lz.compress(data);
        const decompressed = lz.decompress(compressed);
        assert.deepStrictEqual(decompressed, data);
    });

    it('should compress and decompress mixed data', () => {
        const data = Buffer.from('ABCABCABCXYZXYZXYZ123123123');
        const compressed = lz.compress(data);
        const decompressed = lz.decompress(compressed);
        assert.deepStrictEqual(decompressed, data);
    });

    it('should throw error for invalid input', () => {
        assert.throws(() => lz.compress('not a buffer'));
        assert.throws(() => lz.decompress('not a buffer'));
    });

    it('should throw error for invalid token', () => {
        const invalidData = Buffer.from([0x02]); // Invalid token type
        assert.throws(() => lz.decompress(invalidData));
    });

    it('should throw error for incomplete match', () => {
        const invalidData = Buffer.from([0x01, 0x01]); // Missing length
        assert.throws(() => lz.decompress(invalidData));
    });

    it('should throw error for invalid offset', () => {
        const invalidData = Buffer.from([0x01, 0x05, 0x01]); // Offset 5 with empty output
        assert.throws(() => lz.decompress(invalidData));
    });
}); 