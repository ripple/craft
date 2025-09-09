# Local CI Execution

This document explains how to run all GitHub Actions CI commands locally for easy development and testing.

## Quick Start

The fastest way to run CI checks locally:

```shell
# Quick CI check (most important validations)
cargo ci-check
```

## Available Commands

### Core CI Commands

| Command          | Description                             | GitHub Actions Equivalent          |
|------------------|-----------------------------------------|------------------------------------|
| `cargo ci-check` | Quick CI checks (essential validations) | Most important steps from all jobs |
| `cargo ci-full`  | Full CI checks (includes slower tests)  | Complete workflow                  |

### Individual CI Steps

| Command                      | Description               | GitHub Actions Job                   |
|------------------------------|---------------------------|--------------------------------------|
| `cargo pre-commit-check`     | Run pre-commit hooks      | `pre-commit` job                     |
| `cargo clippy-all`           | Clippy on all targets     | `clippy_linting` job                 |
| `cargo fmt-check`            | Format check              | `rustfmt` job                        |
| `cargo wasm-exports-check`   | Check WASM exports        | `clippy_linting` job (exports check) |
| `cargo host-functions-audit` | Audit host functions      | `host_function_audit` job            |
| `cargo markdown-test`        | Test markdown code blocks | `run-markdown` job                   |
| `cargo e2e-test`             | E2E integration tests     | `e2e-tests` job                      |
| `cargo build-all`            | Build all projects        | `build_and_test` job                 |
| `cargo test-all`             | Run all tests             | `build_and_test` job                 |

### Build Commands

| Command              | Description                 |
|----------------------|-----------------------------|
| `cargo build-native` | Build native workspace only |
| `cargo build-wasm`   | Build WASM targets only     |

## Usage Methods

### 1. Cargo Commands (Recommended)

```shell
# Quick check before committing
cargo ci-check

# Full validation (before pushing)
cargo ci-full

# Individual checks
cargo clippy-all
cargo fmt-check
cargo wasm-exports-check
cargo pre-commit-check
cargo host-functions-audit
cargo markdown-test
cargo e2e-test
```

### 2. Direct Cargo Commands

You can also use the underlying cargo commands directly:

```shell
# Build all projects (native and WASM)
cargo build-all

# Run all tests
cargo test-all

# Build and test together
cargo build-all && cargo test-all
```

## Integration with `cargo build`

The CI tools are integrated as workspace members, so they're built when you run:

```shell
cargo build --workspace
```

This ensures the CI tools are always available after building your project.

## What Each Check Does

### Quick CI Check (`cargo ci-check`)

- ✅ Rust formatting
- ✅ Clippy (native workspace)
- ✅ Clippy (WASM projects)
- ✅ WASM exports validation
- ✅ Build native workspace
- ✅ Build WASM targets
- ✅ Run native tests

### Full CI Check (`cargo ci-full`)

All quick checks plus:

- ✅ Pre-commit hooks
- ✅ Host functions audit
- ✅ Craft build examples
- ✅ Markdown code block tests
- ✅ E2E integration tests

## Prerequisites

### Required

- Rust toolchain (stable)
- `wasm32-unknown-unknown` target (auto-installed)

### Optional

- **pre-commit**: `pip install pre-commit && pre-commit install`
- **Node.js**: For host functions audit
- **Docker**: For some E2E tests

## Troubleshooting

### Missing wasm32 target

The scripts automatically install the WASM target:

```shell
rustup target add wasm32-unknown-unknown
```

### Pre-commit not found

Install pre-commit:

```shell
pip install pre-commit
pre-commit install
```

### Node.js not found

The host functions audit will be skipped if Node.js isn't available. This is optional for most development.

### Build failures

If CI tools fail to build, try:

```shell
cargo clean
cargo build --package ci-tools
```

## Environment Variables

The CI tools respect the same environment variables as GitHub Actions:

- `RUSTFLAGS="-Dwarnings"` - Treat warnings as errors (set in `.cargo/config.toml`)

## Performance Tips

- Use `cargo ci-check` for regular development
- Use `cargo ci-full` before pushing to main
- Individual commands for debugging specific issues
- Use `cargo build-native` for fastest native-only builds during development

## Files Created

This setup adds the following files to your project:

- `.cargo/config.toml` - Cargo aliases for CI commands
- `ci-tools/` - Rust binaries for CI tasks
- `CI-LOCAL.md` - This documentation

All build functionality is now integrated through cargo aliases for consistency.
