# Apex Workshop Notes

## Getting Started

1. Install Rust (https://rust-lang.org/tools/install)
1. Install Docker (https://docs.docker.com/engine/install)
1. Clone the repo at https://github.com/ripple/craft
1. Type `craft` to start the CLI and install dependencies

## Running an Example

We will be walking through the `kyc` example. This example will only allow the escrow to be finished if the destination has created a "Terms and Conditions" example to itself.

### Build the example

```sh
% craft
> What would you like to do? Build WASM module
Configuring WASM build settings...
> Select WASM project: kyc
> Select build mode: Release (optimized, no debug info)
> Select optimization level: Aggressive (-Oz: optimize aggressively for size)
Building WASM module...
Running cargo build...
args: ["build", "--target", "wasm32-unknown-unknown", "--release"]

    Finished `release` profile [optimized] target(s) in 0.03s


Build completed successfully!

WASM file location:
/Users/mvadari/Documents/craft/projects/kyc/target/wasm32-unknown-unknown/release/kyc.wasm
Size: 1198 bytes
WASM Fingerprint: wPLUGW1ivhninAUPQCi2VpNiW5GjBkJjeG
> Would you like to export the WASM as hex (copied to clipboard)? No
Optimizing WASM module...
Optimization complete!
> What would you like to do next? Exit
```

### Run the example

```sh
# start Docker
node reference/js/deploy_sample_standalone.js kyc
# Record the Origin, Destination, and Destination Secret from the output above
node reference/js/finish_escrow_kyc.js [DestinationAccount] [DestinationSecret] [OriginAccount] [EscrowSequence]
```
