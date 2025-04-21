#!/usr/bin/env node

const fs = require('fs');
const { program } = require('commander');
const rle = require('./rle');
const lz = require('./lz');

program
    .name('compressor')
    .description('A compression tool implementing RLE and LZ77 algorithms')
    .version('1.0.0');

// Helper function to handle file operations
const handleFileOperation = (operation, input, output, algorithm) => {
    try {
        // Check if input file exists
        if (!fs.existsSync(input)) {
            throw new Error(`Input file not found: ${input}`);
        }

        // Read input file
        const data = fs.readFileSync(input);
        let result;

        // Perform compression/decompression
        if (algorithm === 'rle') {
            result = operation === 'compress' ? rle.compress(data) : rle.decompress(data);
        } else if (algorithm === 'lz') {
            result = operation === 'compress' ? lz.compress(data) : lz.decompress(data);
        }

        // Write output file
        fs.writeFileSync(output, result);
        console.log(`${operation === 'compress' ? 'Compressed' : 'Decompressed'} ${input} to ${output}`);
    } catch (error) {
        console.error(`Error: ${error.message}`);
        process.exit(1);
    }
};

// Compress command
program
    .command('compress <input> <output>')
    .description('Compress a file using RLE or LZ77 algorithm')
    .option('--rle', 'Use RLE compression')
    .option('--lz', 'Use LZ77 compression')
    .action((input, output, options) => {
        if (!options.rle && !options.lz) {
            console.error('Please specify compression algorithm (--rle or --lz)');
            process.exit(1);
        }
        handleFileOperation('compress', input, output, options.rle ? 'rle' : 'lz');
    });

// Decompress command
program
    .command('decompress <input> <output>')
    .description('Decompress a file using RLE or LZ77 algorithm')
    .option('--rle', 'Use RLE decompression')
    .option('--lz', 'Use LZ77 decompression')
    .action((input, output, options) => {
        if (!options.rle && !options.lz) {
            console.error('Please specify decompression algorithm (--rle or --lz)');
            process.exit(1);
        }
        handleFileOperation('decompress', input, output, options.rle ? 'rle' : 'lz');
    });

// Add help command
program.addHelpCommand('help [command]', 'Display help for command');

// Parse arguments
program.parse(process.argv);