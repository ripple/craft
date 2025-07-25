# `craft`

An interactive CLI tool for building and testing WASM modules for the XRP Ledger.

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
cargo install --path .
```

To update the tool, use the same command.

## Requirements

- Rust
- Cargo (with rustup)
- Docker (required for running rippled)

### Installing Docker

Docker is required to run the rippled server. You have two options:

#### Option 1: Colima (for macOS)

Colima is a lightweight, open-source Docker runtime that craft can install automatically:

```bash
craft docker install  # Installs Colima via Homebrew
```

Colima uses less memory and CPU, starts quickly, and works seamlessly with standard Docker commands. It is free to use, requires no login, and has no licensing restrictions.

#### Option 2: Docker Desktop

Traditional option with GUI:

- **macOS**: https://docs.docker.com/desktop/install/mac-install/
- **Windows**: https://docs.docker.com/desktop/install/windows-install/
- **Linux**: https://docs.docker.com/engine/install/

After installation, ensure Docker is running before using rippled-related commands.

## Usage

Use specific commands:

```bash
craft
```

Or, run the tool without any arguments for an interactive experience:

```bash
craft
craft start-rippled   # Start rippled in Docker container
craft list-rippled    # List rippled Docker containers
craft stop-rippled    # Stop the rippled container
craft advance-ledger  # Advance the ledger in stand-alone mode
craft docker          # Manage Docker runtime (install/start/stop/status)
craft open-explorer   # Open the XRPL Explorer
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
The host_functions_test project includes fixtures for testing various host function capabilities:

- **Success Case** (`projects/host_functions_test/fixtures/success/`): Tests successful execution of 26 host functions

### Cloning the Repository

To clone this repository, use:

```bash
git clone git@github.com:ripple/craft.git
```

## Managing rippled

The `craft` tool uses Docker to manage a `rippled` instance. If Docker is not installed, craft can automatically install Colima (a lightweight Docker runtime) for you:

```bash
# Check Docker status and install if needed
craft docker          # Shows status
craft docker install  # Installs Colima (lightweight Docker)
craft docker start    # Starts Colima
craft docker stop     # Stops Colima

# Once Docker is running, manage rippled:
craft start-rippled   # Start rippled container (background mode)
craft start-rippled --foreground  # With visible console output
craft list-rippled    # List running rippled containers
craft stop-rippled    # Stop the rippled container
```

The tool uses the Docker image `legleux/rippled_smart_escrow:bb9bb5f5` which includes support for smart escrows.

**Note**: If Docker is not installed when you run `craft start-rippled`, it will offer to install Colima automatically.

### Docker Commands

You can also manage the container directly with Docker:

```bash
# View logs
docker logs -f craft-rippled

# Stop container
docker stop craft-rippled

# Remove container
docker rm craft-rippled
```

### Ports

- API/WebSocket: `http://localhost:6006`
- Peer Protocol: `localhost:51235`
- Admin API: `localhost:5005`

## Running the XRPL Explorer

The `craft` tool includes commands to open the XRPL Explorer:

```bash
# Open the Explorer
craft open-explorer
```

## WASM Host

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
cargo run -p wasm-host -- --wasm-file path/to/your/module.wasm --test-case success --project <project_name>
```

### Command Line Options

- `--wasm-file <PATH>`: Path to the WebAssembly module to test
- `--wasm-path <PATH>`: (Alias for --wasm-file for backward compatibility)
- `--test-case <CASE>`: Test case to run (defaults to `success`)
- `--project <NAME>`: Project name (required)
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

```
-------------------------------------------------
| WASM FUNCTION EXECUTION ERROR                 |
-------------------------------------------------
| Function:  finish                             |
| Test Case: failure                            |
| Error:     WASM function execution error      |
-------------------------------------------------
```
