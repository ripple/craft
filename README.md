# `craft`

An interactive CLI tool for building and testing WASM modules.

## Installation

```bash
cargo install --path .
```

To update the tool, use the same command.

## Requirements

- Rust
- Cargo (with rustup)

## Usage

Run the tool without any arguments for an interactive experience:

```bash
craft
```

Or use specific commands:

```bash
craft build           # Build a WASM module
craft configure       # Configure build settings
craft export-hex      # Export WASM as hex
craft setup-wee-alloc # Setup wee_alloc for smaller binary size
craft test           # Test a WASM smart contract
```

### Testing WASM Modules

The `test` command provides an interactive environment for testing your WASM modules:

```bash
craft test
```

## Project Structure

Organize your WASM modules in a `projects` directory:

```
.
├── projects/
│   └── helloworld/      # Example
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
└── ...
```

The tool automatically detects WASM projects in the `projects` directory.
