# xrpl-address-macro

A compile-time macro for converting XRPL classic addresses (r-addresses) to 20-byte arrays.

## Features

- **Zero runtime overhead**: Address decoding happens at compile time
- **Type safe**: Invalid addresses cause compilation errors
- **No binary bloat**: The final WASM contains only the raw 20-byte array, no decoding logic

## Usage

```rust
use xrpl_address_macro::r_address;

// Convert r-address to [u8; 20] at compile time
const ACCOUNT: [u8; 20] = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");

// Multiple accounts can be defined
const NOTARY: [u8; 20] = r_address!("rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH");
const ADMIN: [u8; 20] = r_address!("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn");
```

## Comparison with Build Script Approach

### Using this macro (compile-time constant):
```rust
use xrpl_address_macro::r_address;
const NOTARY_ACCOUNT: [u8; 20] = r_address!("rPPL...");
```

### Using build script (environment variable):
```rust
// build.rs decodes NOTARY_ACCOUNT_R env var at build time
include!(concat!(env!("OUT_DIR"), "/notary_generated.rs"));
```

Both approaches result in the same efficient WASM output - the choice depends on whether you need:
- **Macro**: Fixed addresses known at development time
- **Build script**: Addresses configurable via environment variables at build time
