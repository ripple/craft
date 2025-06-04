# Reference Implementations

This directory contains reference implementations and specifications that guide our WASM implementations of XRPL features.

## Utility Scripts

This directory contains utility scripts (in JavaScript) for common XRPL operations.

### `finish_escrow`

A utility script for finishing an escrow transaction. This script creates and submits an EscrowFinish transaction.

Example usage:
```bash
node reference/js/finish_escrow rPPLRQwB3KGvpfDMABZucA8ifJJcvQhHD3 sEd7u9QQjspQBNxGbXZLc2NQjpKUE1L rnNTgUoCakpcSmY3yiHyBi6zusSQdU3Vzy 461231
```

Where:
- First argument: Account that will send the EscrowFinish transaction
- Second argument: Secret key for that Account
- Third argument: Owner (Account that previously funded the escrow with EscrowCreate)
- Fourth argument: Escrow sequence number (Offer Sequence)

For more information about EscrowFinish transactions, see the [XRPL documentation for EscrowFinish](https://xrpl.org/docs/references/protocol/transactions/types/escrowfinish).
