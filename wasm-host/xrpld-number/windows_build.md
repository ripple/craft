# Building xrpld-number on Windows

This guide explains how to build the xrpld-number Rust library on Windows, which requires some additional setup due to the C++ dependencies.

## Prerequisites

- **Rust** installed via [rustup](https://rustup.rs/)
- **Visual Studio** or **Visual Studio Build Tools** with C++ support
- **Git** for downloading vcpkg

## The Issue

The underlying C++ Number library uses 128-bit integers (`uint128_t`). On Windows with MSVC, this type doesn't exist natively, so the code falls back to using `boost::multiprecision::uint128_t` from the Boost library.

## Solution: Using vcpkg

vcpkg is Microsoft's official C++ package manager and provides the cleanest integration.

### Step 1: Install vcpkg

```cmd
# Clone vcpkg to a permanent location (e.g., C:\vcpkg)
git clone https://github.com/Microsoft/vcpkg.git C:\vcpkg
cd C:\vcpkg

# Bootstrap vcpkg
.\bootstrap-vcpkg.bat

# Integrate with Visual Studio (optional but recommended)
.\vcpkg integrate install
```

### Step 2: Install Boost

```cmd
# Install boost-multiprecision (the only Boost component we need)
.\vcpkg install boost-multiprecision

# For 64-bit builds (recommended)
.\vcpkg install boost-multiprecision:x64-windows
```

### Step 3: Configure Environment

Set the CMAKE_TOOLCHAIN_FILE environment variable so the build system can find vcpkg packages:

```cmd
# Set permanently (replace with your actual vcpkg path)
setx CMAKE_TOOLCHAIN_FILE "C:\vcpkg\scripts\buildsystems\vcpkg.cmake"
```

Or temporarily for the current session:
```cmd
set CMAKE_TOOLCHAIN_FILE=C:\vcpkg\scripts\buildsystems\vcpkg.cmake
```

### Step 4: Build the Rust Library

Now you can build normally:

```cmd
cd path\to\xrpld-number
cargo build
```

### Step 5: Run Examples and Tests

```cmd
# Run the example
cargo run --example hello_number

# Run tests
cargo test
```


## Troubleshooting

### "Could not find Boost" Error

```
CMake Error at CMakeLists.txt:X (find_package):
  Could not find a package configuration file provided by "Boost"
```

**Solution:** Make sure vcpkg is properly integrated and `CMAKE_TOOLCHAIN_FILE` is set correctly.

### Linking Errors

If you get linking errors related to Boost:

1. Verify the correct architecture is installed:
   ```cmd
   .\vcpkg list boost-multiprecision
   ```

2. Ensure Visual Studio can find the libraries:
   ```cmd
   .\vcpkg integrate install
   ```

### Build Script Issues

If the Rust build script fails to find compilers:

1. Make sure Visual Studio Build Tools are installed with C++ support
2. Try running from a "Developer Command Prompt" or "Developer PowerShell"

### Performance Note

The first build will take longer as it compiles the C++ code. Subsequent builds will be faster due to Rust's incremental compilation.

## Verification

To verify everything is working correctly:

```cmd
# Build and run tests
cargo test

# Run the comprehensive example
cargo run --example hello_number
```

You should see output demonstrating high-precision arithmetic operations.


## Getting Help

If you encounter issues:

1. Check that all prerequisites are installed correctly
2. Verify vcpkg integration with `vcpkg integrate install`
3. Try building from a Visual Studio Developer Command Prompt

The library should build successfully on Windows following this guide!