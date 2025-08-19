# Notary Escrow FinishFunction

This WebAssembly module implements a notary-based escrow finish condition. It verifies that only a designated notary
account is allowed to finish the escrow.

### How it works

The contract checks whether the account submitting EscrowFinish matches the embedded notary account. If it matches, it
returns 1 (allow), otherwise 0 (deny).

### Function

`finish() -> i32` — returns 1 to allow finishing the escrow, 0 to reject (deny finishing). On host errors, the function
returns a non-zero error code from the host.

## Prerequisites

- Rust toolchain with `wasm32-unknown-unknown` target
- Node.js 18+
- Dependencies installed in `reference/js`:

```bash
cd reference/js
npm install
```

## Step-by-step: Use on WASM Devnet

This guide uses the public Devnet WASM endpoint at `wss://wasm.devnet.rippletest.net:51233` and the helper scripts in
`reference/js`.

### 1) Create a notary account (funded via faucet)

Use the faucet helper script. It prints export lines you can copy/paste.

```bash
cd reference/js
node faucet.js
# Copy the printed export lines into your shell:
# export NOTARY_ADDRESS=...
# export NOTARY_SEED=...
```

Export them for convenience (replace with your printed values):

```bash
export NOTARY_ADDRESS=rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh
```

### 2) Build the notary WASM with your classic address

The build embeds the 20-byte AccountID for the provided r-address.

```bash
NOTARY_ACCOUNT_R=$NOTARY_ADDRESS \
  cargo build
NOTARY_ACCOUNT_R=$NOTARY_ADDRESS \
  cargo build --target wasm32-unknown-unknown
NOTARY_ACCOUNT_R=$NOTARY_ADDRESS \
  cargo build --target wasm32-unknown-unknown --release
```

Artifact:

```
projects/examples/smart-escrows/notary/target/wasm32-unknown-unknown/release/notary.wasm
```

### 3) Deploy an escrow using your FinishFunction on Devnet

Use the helper to deploy an escrow that references your compiled `FinishFunction`.

```bash
cd ../../reference/js
node deploy_sample.js notary
```

This will:

- Connect to WASM Devnet
- Create and fund two wallets (Origin and Destination)
- Create an EscrowCreate transaction with your compiled `FinishFunction`
- Print the transaction result, including `tx_json.Sequence`

Record the following from the output:

- Origin (Owner) address: printed as “Account 1 - Address: ...”
- OfferSequence: from the EscrowCreate `tx_json.Sequence`

For convenience:

```bash
export OWNER_ADDRESS=<Account 1 Address printed by deploy script>
export OFFER_SEQUENCE=<Sequence printed in tx_json>
```

### 4) Finish the escrow as the notary

Submit `EscrowFinish` from the notary account you created in step 1:

```bash
node finish_escrow.js $NOTARY_ADDRESS $NOTARY_SEED $OWNER_ADDRESS $OFFER_SEQUENCE
```

Expected result: `tesSUCCESS` and “Escrow finished successfully!”. If you try to finish from a different account, you
should get `tecNO_PERMISSION` due to the notary check.

## Local testing with wasm-host (optional)

You can also run the WASM locally with the included host emulator:

```bash
cd ../../../../wasm-host
cargo run -- --wasm-file ../projects/examples/smart-escrows/notary/target/wasm32-unknown-unknown/release/notary.wasm --project examples/smart-escrows/notary
```

## Modifying the notary account

Provide a classic address (r...) at build time via `NOTARY_ACCOUNT_R`. The build script verifies Base58 checksum and
embeds the 20-byte AccountID. If unset, it defaults to the Devnet master account `rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh` (
for testing only).

```bash
NOTARY_ACCOUNT_R=rPPLRQwB3KGvpfDMABZucA8ifJJcvQhHD3 \
  cargo build --target wasm32-unknown-unknown --release
```

## Notes

- The contract compares raw 20-byte AccountIDs. Classic addresses are decoded at build-time only.
- Make sure `NOTARY_ACCOUNT_R` matches the account you’ll use in step 4 to submit `EscrowFinish`.
