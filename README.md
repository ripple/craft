# `craft`

An interactive CLI tool for building and testing WASM modules for the XRP Ledger.

## Table of Contents

- [Installation](#installation)
- [Requirements](#requirements)
- [Usage](#usage)
- [Command-Line Options](#command-line-options)
- [Project Structure](#project-structure)
- [WASM Host Testing Tool](#wasm-host-testing-tool)
- [Reference Submodules](#reference-submodules)
- [Managing rippled](#managing-rippled)
- [Running the XRPL Explorer](#running-the-xrpl-explorer)

## Installation

```bash
cargo install --path .
```

To update the tool, use the same command.

## Requirements

- Rust
- Cargo (with rustup)
- WasmEdge

### Installing WasmEdge

If you don't already have WasmEdge, you can install it:

```bash
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
```

After installation, source the updated environment variables:

```bash
source ~/.zshenv  # For zsh users
# OR
source ~/.bashrc  # For bash users
```

To verify the installation:

```bash
which wasmedge
```

If you encounter any dynamic library loading errors when running WASM tests, set the library path:

```bash
# For macOS
export DYLD_LIBRARY_PATH=$DYLD_LIBRARY_PATH:~/.wasmedge/lib

# For Linux
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:~/.wasmedge/lib
```

## Usage

Use specific commands:

```bash
craft build --project <name> [--mode <debug|release>] [--opt <none|small|aggressive>] # Build a WASM module
craft test            # Test a WASM module
craft start-rippled   # Check if rippled is running and start it if needed
craft list-rippled    # List and manage running rippled processes
craft start-explorer  # Set up and run the XRPL Explorer
```

Or, run the tool without any arguments for an interactive experience:

```bash
craft
```

### Command-Line Options

Currently, the `craft` tool primarily uses interactive prompts to gather information such as build mode, optimization level, and project selection.

- **Build**: Non-interactive build with options:

  ```bash
  craft build [project-name] [--mode <debug|release>] [--opt <none|small|aggressive>]
  ```

  Options:

  - `project-name` Name of the project subfolder under `projects/` (positional argument).
  - `--mode, -m` Build mode (`debug` or `release`). Default: `release`.
  - `--opt, -O` Optimization level (`none`, `small`, `aggressive`). Default: `small`.

  Example:

  ```bash
  craft build notary --mode debug --opt aggressive
  ```

The `test` command supports direct command-line options:

```bash
craft test --function <name>  # Test a specific function in your WASM module
```

#### Non-Interactive Mode

For scripting purposes, you may want to specify options directly without interactive prompts. If there are specific options you'd like to set via command line (for example: `craft build --mode release --opt-level small`), please open a GitHub issue to let us know which interactive prompts you'd like to bypass.

### Testing WASM Modules

The `test` command provides an interactive environment for testing your WASM modules:

```bash
craft test
```

## Project Structure

Organize your WASM modules in the `projects` directory:

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

# WASM Host Testing Tool

This tool provides a testing environment for XLS-100d compliant WebAssembly modules. It simulates the host environment that will execute escrow finish conditions on the XRPL.

## Purpose

The wasm-host tool:

1. Loads and executes WebAssembly modules
2. Provides test transaction and ledger object data
3. Calls the `finish` function as specified in XLS-100d
4. Reports execution results and any errors

## Test Fixtures

The tool includes a set of test fixtures in the `fixtures/escrow` directory. Currently, these fixtures are specific to the `notary` project. The intent is to generalize or reuse for future projects.

### Success Case (`fixtures/escrow/success/`)

- `tx.json`: Transaction with the correct notary account
- `ledger_object.json`: Corresponding escrow object

### Failure Case (`fixtures/escrow/failure/`)

- `tx.json`: Transaction with an incorrect notary account
- `ledger_object.json`: Corresponding escrow object

## Reference Submodules

See [reference/README.md](reference/README.md) for details on using and updating the reference implementations.

### 1. rippled

Located at `reference/rippled`, this provides the authoritative XRPL server implementation.

### 2. XRPL Explorer

Located at `reference/explorer`, this provides a web interface for exploring XRPL transactions and data.

### Cloning the Repository with Submodules

To clone this repository including all submodules, use:

```bash
git clone --recurse-submodules git@github.com:ripple/craft.git
```

Or if you've already cloned the repository without submodules:

```bash
git submodule update --init --recursive
```

### Updating Submodules

To update all submodules to their latest versions:

```bash
git submodule update --remote
```

## Managing rippled

The `craft` tool includes commands to manage a local `rippled` instance:

```bash
# Check if rippled is running and start it if not (background mode)
craft start-rippled

# Start rippled with visible console output (can be terminated with Ctrl+C)
craft start-rippled --foreground

# List running rippled processes and show how to terminate them
craft list-rippled
```

To terminate `rippled`:

```bash
killall rippled
```

## Running the XRPL Explorer

The `craft` tool includes commands to open the XRPL Explorer:

```bash
# Open the Explorer
craft open-explorer
```

## WASM Host Testing

### Direct Usage

From the `wasm-host` directory:

```bash
# Run with success test case
cargo run -- --wasm-file ../path/to/your/module.wasm --test-case success

# Run with failure test case
cargo run -- --wasm-file ../path/to/your/module.wasm --test-case failure
```

From any workspace directory:

```bash
cargo run -p wasm-host -- --wasm-file path/to/your/module.wasm --test-case success
```

### Command Line Options

- `--wasm-file <PATH>`: Path to the WebAssembly module to test
- `--wasm-path <PATH>`: (Alias for --wasm-file for backward compatibility)
- `--test-case <CASE>`: Test case to run (success/failure)
- `--verbose`: Enable detailed logging
- `-h, --help`: Show help information

### Debugging with Verbose Mode

To see detailed execution information, including memory allocation, data processing, and function execution steps, use the `--verbose` flag:

```bash
cargo run -p wasm-host -- --wasm-file path/to/module.wasm --test-case success --verbose
```

The verbose output may include:

- Memory allocation details
- JSON data being processed
- Function execution steps
- Results of the execution

Example verbose output:

```
[INFO wasm_host] Starting WasmEdge host application
[INFO wasm_host] Loading WASM module from: path/to/module.wasm
[INFO wasm_host] Target function: finish (XLS-100d)
[INFO wasm_host] Using test case: success
[DEBUG wasm_host] Initializing WasiModule
[DEBUG wasm_host] WasiModule initialized successfully
[INFO wasm_host::vm] Executing WASM function: finish
[DEBUG wasm_host::vm] TX data size: 610 bytes, LO data size: 919 bytes
[INFO wasm_host::vm] Allocating memory for transaction data
[DEBUG wasm_host::vm] Allocated memory at address: 0x110008
...
```

### Integration with `craft`

The wasm-host tool is typically used through the `craft test` command, which provides an interactive interface for selecting test cases:

```bash
# Test a WASM module
craft test

# Test with verbose output
RUST_LOG=debug craft test
```

The interactive interface will prompt you to select:

1. Test case (success/failure)
2. Other build and test options

## Test Data

The tool provides test data that simulates:

1. An EscrowFinish transaction
2. An Escrow ledger object

This data is used to test the module's `finish` function implementation.

### Adding New Test Cases

To add new test cases:

1. Create a new directory under `fixtures/escrow/`
2. Add `tx.json` and `ledger_object.json` files
3. Update the test case selection in the craft tool

## Error Handling

If the WebAssembly module execution fails, the tool will:

1. Display an error message explaining the failure
2. Show the function name that failed
3. Show the test case being run
4. Provide context about the error
5. Exit with a non-zero status code

Example error output:

```
-------------------------------------------------
| WASM FUNCTION EXECUTION ERROR                 |
-------------------------------------------------
| Function:  finish                             |
| Test Case: failure                            |
| Error:     WASM function execution error      |
-------------------------------------------------
```
