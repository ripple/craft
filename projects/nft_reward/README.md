# NFT Reward Escrow

This project implements a Smart Escrow FinishFunction that only returns `true` when the escrow destination owns a specific NFT (NFToken object).

## Overview

The escrow can only be finished if the destination account owns a particular NFT specified by its NFTokenID. This creates a mechanism where the escrow funds can only be claimed when an account has ownership of a specific NFT.

## Implementation Details

The `ready()` function:
1. Gets the escrow destination account
2. For the specified NFTokenID:
   - Extracts its low 96 bits
   - Constructs the NFTokenPage ID by concatenating the destination account and the low 96 bits
   - Checks up to 3 possible pages where the NFT could be located
   - For each page:
     - Retrieves the NFTokenPage entry from the ledger
     - Parses the page to verify if it contains the specific NFT
3. Returns `true` only if the NFT is found in one of the destination's NFTokenPages

## NFTokenPage ID Format

NFTokenPage identifiers are constructed to allow a more efficient paging structure, ideally suited for NFToken objects.

The identifier of an NFTokenPage is derived by concatenating the 160-bit AccountID of the owner of the page, followed by a 96 bit value that indicates whether a particular NFTokenID can be contained in this page.

More specifically, a NFToken with the NFTokenID value A can be included in a page with NFTokenPage ID B if and only if low96(A) >= low96(B).

This uses a function low96(x) which returns the low 96 bits of a 256-bit value. For example, applying the low96 function to the NFTokenID of 000B013A95F14B0044F78A264E41713C64B5F89242540EE208C3098E00000D65 returns the value 42540EE208C3098E00000D65.

https://xrpl.org/docs/references/protocol/ledger-data/ledger-entry-types/nftokenpage

## Finding NFTokens

To find a specific NFToken:
1. Know its NFTokenID and current owner
2. Compute the NFTokenPage ID as described above
3. Search for a ledger entry whose identifier is less than or equal to that value
4. If that entry does not exist or is not an NFTokenPage, that account does not own that NFToken

## Configuration

The NFTokenID that must be owned by the destination is specified in the `REQUIRED_NFT_ID` constant in `src/lib.rs`. You should replace this with the actual NFTokenID you want to use.

## Building

To build the project:
```bash
cargo build --release
```

The compiled WebAssembly module will be available in `target/wasm32-unknown-unknown/release/nft_reward.wasm` 