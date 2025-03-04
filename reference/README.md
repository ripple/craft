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
