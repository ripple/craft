# Craft - WASM Smart Contract Build Tool

An interactive CLI tool for building and testing WASM smart contracts for blockchain deployment.

## Features

- Interactive CLI interface - no need to memorize commands
- Flexible WASM target selection (wasm32-unknown-unknown, wasm32-wasi-preview1)
- Build mode selection (debug/release)
- WASM optimization with wasm-opt
- Automatic wasm-opt installation
- wee_alloc integration for smaller binary sizes
- Automatic WASM to hex conversion with clipboard support
- Automatic WASM target installation
- Integrated WASM testing environment
- Interactive contract function testing

## Installation

```bash
cargo install --path .
```

To update the tool, use the same command.

## Requirements

- Rust and Cargo (with rustup)
- For optimization: binaryen (wasm-opt) - will be installed automatically if needed
- For clipboard support on macOS: pbcopy (built-in)
- For testing: WasmEdge runtime - will be used automatically

## Usage

Simply run the tool without any arguments for an interactive experience:

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

### Testing WASM Contracts

The `test` command provides an interactive environment for testing your WASM smart contracts:

```bash
# Test with default function (get_greeting)
craft test

# Test a specific function
craft test -f set_greeting
craft test -f reset_to_default
```

Available test functions:
- `get_greeting`: Get the current greeting
- `set_greeting`: Set a new greeting
- `reset_to_default`: Reset to "Hello, world!"

The testing environment:
1. Automatically builds your contract if needed
2. Loads it into a WasmEdge VM
3. Creates a new contract instance
4. Executes the specified function
5. Displays the results

## Build Configuration

The tool will guide you through configuring:

1. WASM Target:
   - wasm32-unknown-unknown (recommended for most blockchain deployments)
   - wasm32-wasi-preview1 (for WASI compatible environments)
   - Targets will be automatically installed if not present

2. Build Mode:
   - Release (optimized, no debug info)
   - Debug (includes debug info)

3. Optimization Level:
   - None (no optimization)
   - Small (-Os: optimize for size)
   - Aggressive (-Oz: optimize aggressively for size)

4. wee_alloc Integration:
   - Optional integration for smaller binary sizes

## Project Structure

For best results, organize your WASM smart contracts in a `projects` directory:

```
.
├── projects/
│   └── helloworld/      # Example contract
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
└── ...
```

The tool will automatically detect WASM projects in the `projects` directory.

## Troubleshooting

If you encounter any issues:

1. Make sure you have rustup installed and it's up to date
2. Ensure you're in a directory with a valid Rust project
3. Check that your Rust project has a valid Cargo.toml file
4. The tool will automatically install required WASM targets if they're missing
5. For testing issues, ensure your contract implements the expected interface
