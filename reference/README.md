# Reference Implementations

This directory contains reference implementations and specifications that guide our WASM implementations of XRPL features.

## `rippled`

This is included as a git submodule pointing to [intelliot/rippled@pseudo-code](https://github.com/intelliot/rippled/tree/pseudo-code).

### Purpose

The following are proposed (not yet implemented).

1. **Specification Reference**: Provides authoritative implementation details for XRPL features
2. **Test Vectors**: Source of test data and scenarios
3. **Validation**: Helps ensure our WASM implementations match the spec
4. **Documentation**: Access to detailed technical documentation and design rationales

### Updating the Reference

To update the rippled reference to the latest code:

```bash
# Update the submodule
git submodule update --remote reference/rippled

# Commit the update
git add reference/rippled
git commit -m "Update rippled reference to latest"
```

### First-time Setup

When cloning this repository, you'll need to initialize the submodule:

```bash
# During clone
git clone --recursive [repository-url]

# Or after clone
git submodule update --init --recursive
```

### Note

This is a reference implementation only, not a dependency. Our WASM modules are independent implementations that follow the XRPL specifications but are designed specifically for the WebAssembly environment.

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
