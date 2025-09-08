# `craft`

An interactive CLI tool for building and testing [XLS-102 WASM](https://github.com/XRPLF/XRPL-Standards/discussions/303) modules for the XRP Ledger.

## Table of Contents

- [Installation](#installation)
- [Requirements](#requirements)
- [Usage](#usage)
- [Command-Line Options](#command-line-options)
- [Project Structure](#project-structure)
- [WASM Host Testing Tool](#wasm-host-testing-tool)
- [Managing rippled](#managing-rippled)
- [Running the XRPL Explorer](#running-the-xrpl-explorer)

## Installation

```bash
cargo install --path ./craft
```

To update the tool, use the same command.

## Requirements

- Rust
- Cargo (with rustup)
- Docker (recommended for running rippled; optional if you build rippled locally)

### Installing Docker

Docker is recommended to run the rippled server. Alternatively, you can build and run rippled locally by following the BUILD instructions in the rippled repository.

- **macOS**: <https://docs.docker.com/desktop/install/mac-install/>
- **Windows**: <https://docs.docker.com/desktop/install/windows-install/>
- **Linux**: <https://docs.docker.com/engine/install/>

After installation, ensure Docker is running before using rippled-related commands.

## Usage

Use specific commands:

```shell
craft
```

Or, run the tool without any arguments for an interactive experience:

```shell
craft
craft start-rippled   # Start rippled in Docker container
craft list-rippled    # List rippled Docker containers
craft stop-rippled    # Stop the rippled container
craft advance-ledger  # Advance the ledger in stand-alone mode
craft open-explorer   # Open the XRPL Explorer
```

### Command-Line Options

Currently, the `craft` tool primarily uses interactive prompts to gather information such as build mode, optimization level, and project selection.

- **Build**: Non-interactive build with options:

  ```shell
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

```shell
craft test --function <name>  # Test a specific function in your WASM module
```

#### Non-Interactive Mode

For scripting purposes, you may want to specify options directly without interactive prompts. If there are specific options you'd like to set via command line (for example: `craft build --mode release --opt-level small`), please open a GitHub issue to let us know which interactive prompts you'd like to bypass.

### Testing WASM Modules

The `test` command provides an interactive environment for testing your WASM modules:

```shell
craft test
```

## Project Structure

Organize your WASM modules in the `projects` directory:

```text
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

This tool provides a testing environment for [XLS-100](https://github.com/XRPLF/XRPL-Standards/discussions/270) compliant WebAssembly modules. It simulates the host environment that will execute escrow finish conditions on the XRPL.

For details on the WASM VM integration, see [XLS-102: WASM VM Configuration](https://github.com/XRPLF/XRPL-Standards/discussions/303).

## Purpose

The wasm-host tool:

1. Loads and executes WebAssembly modules
2. Provides test transaction and ledger object data
3. Calls the `finish` function as specified in [XLS-100](https://github.com/XRPLF/XRPL-Standards/discussions/270)
4. Reports execution results and any errors

## Test Fixtures

Test fixtures must be placed in `projects/<project>/fixtures/<test_case>/`

This convention co-locates each project's test data with its source code, making projects self-contained.

### Fixture Structure

Each test case directory can contain:

- `tx.json`: Transaction data
- `ledger_object.json`: Current ledger object being tested
- `ledger_header.json`: Ledger header information
- `ledger.json`: Full ledger data
- `nfts.json`: NFT data (if applicable)

### Example Projects with Test Fixtures

#### Notary Project

The notary project includes test fixtures for validating escrow finish conditions:

- **Success Case** (`projects/notary/fixtures/success/`): Tests when the escrow finish condition is met (transaction with the correct notary account)
- **Failure Case** (`projects/notary/fixtures/failure/`): Tests when the escrow finish condition is not met (transaction with an incorrect notary account)

#### Host Functions Test Project

The host_functions_test project includes fixtures for testing various host function capabilities. This project can be found in:

`rippled-tests/host_functions_test/`

- **Success Case** (`rippled-tests/host_functions_test/fixtures/success/`): Tests successful execution of 26 host functions

### Cloning the Repository

To clone this repository, use:

```bash
git clone git@github.com:ripple/craft.git
```

## Managing rippled

The `craft` tool uses Docker to manage a `rippled` instance.

Ensure Docker Desktop is installed and running. Then manage rippled using these commands:

```shell
craft start-rippled                 # Start rippled container (background mode)
craft start-rippled --foreground    # Start with visible console output
craft list-rippled                  # List running rippled containers
craft stop-rippled                  # Stop the rippled container
```

The tool uses the Docker image `legleux/rippled_smart_escrow:bb9bb5f5` which includes support for smart escrows.

**Note**: Ensure Docker is installed and running before using `craft start-rippled`.

### Docker Commands

You can also manage the container directly with Docker:

```shell
# View logs
docker logs -f craft-rippled

# Stop container
docker stop craft-rippled

# Remove container
docker rm craft-rippled
```

### Ports

- Public WebSocket: `ws://localhost:6005`
- Admin WebSocket: `ws://localhost:6006`
- Admin RPC API: `http://localhost:5005`

## Running the XRPL Explorer

The `craft` tool includes commands to open the XRPL Explorer:

```shell
# Open the Explorer
craft open-explorer
```

## WASM Host

### Direct Usage

From the `wasm-host` directory:

```shell
# Run with success test case
cargo run -- --dir path/to/your/project_name --test-case success

# Run with failure test case
cargo run -- --dir path/to/your/project_name --test-case failure
```

From any workspace directory:

```shell
cargo run -p wasm-host -- --dir path/to/your/project_name --test-case success --project <project_name>
```

### Command Line Options

- `--dir <PATH>`: Path to the source code where fixtures are located
- `--test-case <CASE>`: Test case to run (defaults to `success`)
- `--project <NAME>`: Project name (required)
- `--verbose`: Enable detailed logging
- `-h, --help`: Show help information

### Debugging with Verbose Mode

To see detailed execution information, including memory allocation, data processing, and function execution steps, use the `--verbose` flag:

```shell
cargo run -p wasm-host --dir path/to/folder --project project --test-case success --verbose
```

The verbose output may include:

- Memory allocation details
- JSON data being processed
- Function execution steps
- Results of the execution

Example verbose output:

```text
[INFO wasm_host] Starting Wasm host application
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

```shell
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

To add new test cases to a project:

1. Create a new directory under `projects/<project>/fixtures/<test_case>/`
2. Add desired JSON files:
   - `tx.json`: Transaction data
   - `ledger_object.json`: Ledger object being tested
   - `ledger_header.json`: Ledger header information
   - `ledger.json`: Full ledger data
   - `nfts.json`: NFT data (if applicable)
3. Run the test using: `craft test <project> --case <test_case>`

## Error Handling

If the WebAssembly module execution fails, the tool will:

1. Display an error message explaining the failure
2. Show the function name that failed
3. Show the test case being run
4. Provide context about the error
5. Exit with a non-zero status code

Example error output:

```text
-------------------------------------------------
| WASM FUNCTION EXECUTION ERROR                 |
-------------------------------------------------
| Function:  finish                             |
| Test Case: failure                            |
| Error:     WASM function execution error      |
-------------------------------------------------
```

## Rust Documentation

This repository contains multiple Rust crates. You can use rustdoc to generate and view documentation.

### Generate documentation

- Public crates only (recommended):
  - `cargo doc --no-deps -p craft --target-dir target`
  - `cargo doc --no-deps -p xrpl-std --target-dir target`
- Entire workspace:
  - `cargo doc --workspace --no-deps`
- Open docs in your browser:
  - `cargo doc --no-deps --open`

A helper script is included:

```shell
./build-docs.sh
```

This cleans previous docs, builds docs for `craft` and `xrpl-std` (into a shared target/ directory), runs doctests for `xrpl-std`, and prints the path to the rendered docs.

### View the documentation

- After building, open: `target/doc/index.html` to see the docs index
- Direct links:
  - Craft CLI docs: `target/doc/craft/index.html`
  - xrpl-std library: `target/doc/xrpl_std/index.html`
- Or simply run: `cargo doc --open`

### Best practices for writing Rust docs

- Use `//!` for crate- and module-level documentation; use `///` for items (functions, structs, enums)
- Prefer small, runnable examples. For examples that should not run in doctests, use code fences with language modifiers:
  - `rust,no_run` for examples that should compile but not execute
  - `rust,ignore` for examples that should not be compiled
- Use intra-doc links to reference items within a crate, e.g. `[Result](core::result::Result)`
- Test your docs: `cargo test --doc` (per-crate or workspace)
- Hide internal implementation details with `#[doc(hidden)]`
- Feature-gate docs for optional APIs with `#[cfg_attr(doc_cfg, doc(cfg(feature = "...")))]`

### Including external Markdown in rustdoc

You can include standalone Markdown files directly into your crate documentation using `include_str!`:

- Include a crate README as the top-level docs (in `src/lib.rs` or `src/main.rs`):

```rust
#![doc = include_str!("../../README.md")]
```

- Include additional guides as modules shown in docs only:

```rust
/// Additional guides and how-tos
#[cfg(doc)]
pub mod guides {
    /// XRPL Field Access and Locators guide
    #[doc = include_str!("../../docs/FIELD_ACCESS.md")]
    pub mod field_access {}
}
```

In this repository:

- The `xrpl-std` crate already includes its README via `#![doc = include_str!("../README.md")]`
- The guide at `docs/FIELD_ACCESS.md` is included under the rendered docs at `xrpl_std::guides::field_access`

### Notes on code blocks in docs

- Examples that reference unavailable items or host-only APIs are marked as `rust,ignore` to prevent doctest failures
- Prefer `rust` or `rust,no_run` for examples intended to compile
