# WASM Host Testing Tool

This tool provides a testing environment for XLS-100d compliant WebAssembly modules. It simulates the host environment that will execute escrow finish conditions on the XRPL.

## Purpose

The wasm-host tool:
1. Loads and executes WebAssembly modules
2. Provides test transaction and ledger object data
3. Calls the `finish` function as specified in XLS-100d
4. Reports execution results and any errors

## Usage

### Direct Usage

From the `wasm-host` directory:
```bash
cargo run -- --wasm-file ../path/to/your/module.wasm
```

From any workspace directory:
```bash
cargo run -p wasm-host -- --wasm-file path/to/your/module.wasm
```

### Command Line Options

- `--wasm-file <PATH>`: Path to the WebAssembly module to test
- `--wasm-path <PATH>`: (Alias for --wasm-file for backward compatibility)
- `--verbose`: Enable detailed logging
- `-h, --help`: Show help information

### Debugging with Verbose Mode

To see detailed execution information, including memory allocation, data processing, and function execution steps, use the `--verbose` flag:

```bash
# From wasm-host directory
cargo run -- --wasm-file ../projects/notary/target/wasm32-unknown-unknown/release/notary.wasm --verbose

# From workspace root
cargo run -p wasm-host -- --wasm-file projects/notary/target/wasm32-unknown-unknown/release/notary.wasm --verbose
```

The verbose output may include:
- Memory allocation details
- JSON data being processed
- Function execution steps
- Results of the execution

Example verbose output:
```
[INFO wasm_host] Starting WasmEdge host application
[INFO wasm_host] Loading WASM module from: ../projects/notary/target/wasm32-unknown-unknown/release/notary.wasm
[INFO wasm_host] Target function: finish (XLS-100d)
[DEBUG wasm_host] Initializing WasiModule
[DEBUG wasm_host] WasiModule initialized successfully
[INFO wasm_host::vm] Executing WASM function: finish
[DEBUG wasm_host::vm] TX data size: 610 bytes, LO data size: 919 bytes
[INFO wasm_host::vm] Allocating memory for transaction data
[DEBUG wasm_host::vm] Allocated memory at address: 0x110008
...
```

### Integration with `craft`

The wasm-host tool is typically used through the `craft test` command, which handles building and running the tool with appropriate arguments:

```bash
# Test a WASM module
craft test

# Test with verbose output
RUST_LOG=debug craft test
```

## Test Data

The tool provides test data that simulates:
1. An EscrowFinish transaction
2. An Escrow ledger object

This data is used to test the module's `finish` function implementation.

## Error Handling

If the WebAssembly module execution fails, the tool will:
1. Display an error message explaining the failure
2. Show the function name that failed
3. Provide context about the error
4. Exit with a non-zero status code

Example error output:
```
-------------------------------------------------
| WASM FUNCTION EXECUTION ERROR                 |
-------------------------------------------------
| Function: finish                              |
| Error:    WASM function execution error       |
-------------------------------------------------
``` 