# Proposal: WASM Module Fingerprint Specification

## Overview

This document proposes a standard for generating short, unique fingerprints for WebAssembly (WASM) modules. These fingerprints serve as compact identifiers that can be used to:
- Track module versions
- Verify module integrity
- Compare modules for changes
- Reference modules in logs and documentation

## Requirements

A WASM module fingerprint must:
1. Be deterministic (same module always produces same fingerprint)
2. Be sensitive to changes (small changes produce different fingerprints)
3. Be compact (short enough to be human-readable)
4. Be collision-resistant (different modules unlikely to produce same fingerprint)
5. Be easy to compute and verify

## Specification

### Input
- Raw WASM binary module (`.wasm` file)

### Algorithm
1. Compute RIPEMD160 hash of the WASM binary
2. Use all 160 bits (20 bytes) of the hash
3. Prepend the type prefix byte (0x17)
4. Compute checksum:
   - First SHA-256 hash of the prefix + hash
   - Second SHA-256 hash of the first hash result
   - Take first 4 bytes of the second hash
5. Append checksum to the prefix + hash
6. Encode in XRPL Base58 format
   - Uses XRPL's base58 dictionary: `rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz`
   - Result always starts with "w" (for "wasm")

### Output Format
- 35-character string (including the "w" prefix)
- Characters from XRPL's base58 dictionary
- Always starts with "w"
- Example: `wQr9wpGtwK3Mro4jvpKLMmN8LoPqRsTUVWXYZ2abcd`

### Example
```javascript
// Example implementation
const crypto = require('crypto');
const bs58 = require('bs58');

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
    
    // 6. Encode in XRPL Base58
    const xrplAlphabet = 'rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz';
    const fingerprint = bs58.encode(withChecksum, xrplAlphabet);
    
    return fingerprint;
}
```

## Properties

### Uniqueness
- 160-bit fingerprint space (2^160 possibilities)
- Probability of collision in 1 million modules: negligible
- Suitable for identification and cryptographic security

### Change Sensitivity
- Any change to the WASM binary produces a different fingerprint
- Even single-byte changes result in completely different fingerprints
- Preserves full cryptographic properties of RIPEMD160

### Compactness
- 35 characters is still manageable for human readability
- Always starts with "w" (due to 0x17 prefix)
- Similar to XRPL's base58 encoding
- Easy to read, copy, and share

### Error Detection
- Double SHA-256 checksum provides robust error detection
- Same checksum algorithm as used by Bitcoin and XRP Ledger
- Helps catch transmission errors and typos
- Validates fingerprint integrity

## Use Cases

1. **Version Tracking**
   - Track module versions in deployment logs
   - Reference specific module versions in documentation
   - Compare deployed modules with known versions

2. **Integrity Verification**
   - Verify module hasn't been modified
   - Compare fingerprints before and after deployment
   - Detect unauthorized changes

3. **Change Detection**
   - Quickly identify if modules are different
   - Track module evolution over time
   - Document module changes in changelogs

4. **Module Identification**
   - Reference modules in logs and documentation
   - Include in error messages and debugging
   - Use in module registry systems

## Implementation Notes

1. **Performance**
   - RIPEMD160 computation is very fast
   - Double SHA-256 checksum adds negligible overhead
   - Base58 encoding with checksum provides error detection
   - Suitable for real-time fingerprinting
   - Can be computed during build process

2. **Storage**
   - Store fingerprints alongside modules
   - Include in module metadata
   - Use in deployment manifests

3. **Verification**
   - Implement verification tools
   - Compare fingerprints across environments
   - Validate module integrity
   - Verify checksum to detect errors

## Security Considerations

1. **Collision Resistance**
   - 160-bit fingerprint space provides strong collision resistance
   - Suitable for cryptographic security
   - Excellent for version tracking and change detection

2. **Preimage Resistance**
   - Fingerprint reveals no information about module
   - Cannot reconstruct module from fingerprint
   - Safe to share publicly

3. **Error Detection**
   - Double SHA-256 checksum provides robust error detection
   - Same checksum algorithm as Bitcoin and XRP Ledger
   - Helps catch typos and transmission errors
   - Aligns with XRPL's encoding practices

## Future Considerations

1. **Extensions**
   - Add version information
   - Include build metadata
   - Support for module dependencies

2. **Standardization**
   - Share with community
   - Use in WASM tooling
   - Integration with future WASM registries

## References

1. XRPL Base58 Encoding Specification
2. RIPEMD160 Specification
3. Base58Check Encoding
