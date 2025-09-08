# WASM Host Testing Tool

This tool provides a testing environment for FinishFunction WebAssembly modules structured according to [XLS-100d](https://github.com/XRPLF/XRPL-Standards/discussions/270). It simulates the host environment that will execute escrow finish functions on the XRPL.

## Purpose

The wasm-host tool:

1. Loads and executes WebAssembly modules
2. Provides test transaction and ledger object data
3. Calls a specified function in the WASM module (defaults to `finish`)
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

### Example: Notary Project

The notary project includes test fixtures for validating escrow finish conditions:

#### Success Case (`projects/notary/fixtures/success/`)

- `tx.json`: Transaction with the correct notary account
- `ledger_object.json`: Corresponding escrow object

#### Failure Case (`projects/notary/fixtures/failure/`)

- `tx.json`: Transaction with an incorrect notary account
- `ledger_object.json`: Corresponding escrow object

## Usage

### Direct Usage

From the `wasm-host` directory:

```shell
# Run with success test case
cargo run -- --wasm-file ../path/to/your/module.wasm --test-case success --project <project_name>

# Run with a specific function
cargo run -- --wasm-file ../path/to/your/module.wasm --test-case success --project <project_name> --function your_function_name

# Run with failure test case, specify function name "finish"
cargo run -- --wasm-file ../path/to/your/module.wasm --test-case failure --project <project_name> --function finish
```

From any workspace directory:

```shell
cargo run -p wasm-host -- --wasm-file path/to/your/module.wasm --test-case success --project <project_name> --function finish
```

### Command Line Options

- `--dir <PATH>`: Path to the source code (for fixture purposes)
- `--test-case <CASE>`: Test case to run (defaults to `success`)
- `--project <NAME>`: Project name (required)
- `--function <NAME>`: The name of the function to execute in the WASM module, defaults to `finish`
- `--verbose`: Enable detailed logging
- `-h, --help`: Show help information

### Debugging with Verbose Mode

To see detailed execution information, including memory allocation, data processing, and function execution steps, use the `--verbose` flag:

```bash
cargo run -p wasm-host -- --wasm-file path/to/module.wasm --test-case success --verbose
```

The verbose output includes:

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

```
-------------------------------------------------
| WASM FUNCTION EXECUTION ERROR                 |
-------------------------------------------------
| Function:   finish                |
| Test Case:  failure                           |
| Error:      WASM function execution error       |
-------------------------------------------------
```
