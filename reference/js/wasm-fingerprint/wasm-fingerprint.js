#!/usr/bin/env node

const fs = require('fs');
const crypto = require('crypto');
const rippleAddressCodec = require('ripple-address-codec');

function generateFingerprint(wasmBinary) {
    // 1. Compute RIPEMD160
    const hash = crypto.createHash('ripemd160')
        .update(wasmBinary)
        .digest();
    
    // 2. Use all 20 bytes
    const fingerprintBytes = hash;
    
    // 3. Prepend WASM type prefix (0x17)
    const prefix = Buffer.from([0x17]);
    const prefixed = Buffer.concat([prefix, fingerprintBytes]);
    
    // 4. Compute double SHA-256 checksum
    const firstHash = crypto.createHash('sha256')
        .update(prefixed)
        .digest();
    const secondHash = crypto.createHash('sha256')
        .update(firstHash)
        .digest();
    const checksum = secondHash.slice(0, 4);
    
    // 5. Append checksum
    const withChecksum = Buffer.concat([prefixed, checksum]);
    
    // 6. Encode using XRPL's Base58 alphabet via ripple-address-codec
    // The ripple-address-codec uses the XRPL alphabet by default
    const fingerprint = rippleAddressCodec.encode(withChecksum, {
        // No version needed since we already prepended our own prefix
        version: null,
        // Use the ripple alphabet (default)
        alphabet: 'ripple'
    });
    
    return fingerprint;
}

function hexToBuffer(hexString) {
    // Remove any whitespace or 0x prefix and convert to lowercase
    const cleanHex = hexString.replace(/[\s0x]/g, '').toLowerCase();
    
    // Check if the hex string is valid
    if (!/^[0-9a-f]*$/.test(cleanHex)) {
        throw new Error('Invalid hex string');
    }
    
    // Check if the hex string has an even length
    if (cleanHex.length % 2 !== 0) {
        throw new Error(`Hex string must have an even number of characters (got ${cleanHex.length})`);
    }
    
    // Convert hex to buffer
    return Buffer.from(cleanHex, 'hex');
}

function main() {
    if (process.argv.length !== 3) {
        console.error('Usage: node wasm-fingerprint.js <wasm-file-path-or-hex-string>');
        process.exit(1);
    }

    const input = process.argv[2];
    let wasmBinary;

    try {
        // Check if input is a file path
        if (fs.existsSync(input) && !input.match(/^[0-9a-fA-F]+$/)) {
            wasmBinary = fs.readFileSync(input);
        } else {
            // Assume it's a hex string
            wasmBinary = hexToBuffer(input);
        }

        const fingerprint = generateFingerprint(wasmBinary);
        console.log(fingerprint);
    } catch (error) {
        console.error('Error:', error.message);
        process.exit(1);
    }
}

main(); 
