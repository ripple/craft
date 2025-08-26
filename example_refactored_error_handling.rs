// Example demonstrating the refactored error handling with standard Rust Result and thiserror
// This shows how the WASM response code idiom is maintained while using standard error traits

use xrpl_std::host::{Error, Result};
use xrpl_std::host::error_codes::match_result_code;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("=== XRPL-STD Refactored Error Handling Demo ===\n");

    // Example 1: Using standard Result with our custom Error
    println!("1. Standard Result usage:");
    let success_result: Result<String> = Ok("Success!".to_string());
    let error_result: Result<String> = Err(Error::FieldNotFound);

    match success_result {
        Ok(value) => println!("   ✓ Success: {}", value),
        Err(e) => println!("   ✗ Error: {}", e),
    }

    match error_result {
        Ok(value) => println!("   ✓ Success: {}", value),
        Err(e) => println!("   ✗ Error: {} (implements std::error::Error: {})", e, std::error::Error::source(&e).is_none()),
    }

    // Example 2: WASM response code handling (maintains original idiom)
    println!("\n2. WASM response code handling:");
    
    // Simulate positive response code (success with array length)
    let positive_code = 32; // 32 bytes written
    let result = match_result_code(positive_code, || {
        "Data successfully written (32 bytes)".to_string()
    });
    
    match result {
        Ok(msg) => println!("   ✓ Positive code ({}): {}", positive_code, msg),
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // Simulate negative response code (error)
    let negative_code = -2; // FIELD_NOT_FOUND
    let result = match_result_code(negative_code, || {
        "This closure should not execute".to_string()
    });
    
    match result {
        Ok(msg) => println!("   ✓ Success: {}", msg),
        Err(e) => println!("   ✗ Negative code ({}): {}", negative_code, e),
    }

    // Example 3: Error code access (maintains original functionality)
    println!("\n3. Error code access:");
    let error = Error::BufferTooSmall;
    println!("   Error: {}", error);
    println!("   Code: {}", error.code());
    println!("   From code roundtrip: {}", Error::from_code(error.code()).code());

    // Example 4: Standard error trait compatibility
    println!("\n4. Standard error trait compatibility:");
    let error: Box<dyn std::error::Error> = Box::new(Error::InvalidParams);
    println!("   Error as trait object: {}", error);
    println!("   Debug representation: {:?}", Error::InvalidParams);

    // Example 5: Using ? operator (standard Rust error propagation)
    println!("\n5. Standard error propagation with ? operator:");
    match example_function_with_error_propagation() {
        Ok(value) => println!("   ✓ Function succeeded: {}", value),
        Err(e) => println!("   ✗ Function failed: {}", e),
    }

    println!("\n=== Demo Complete ===");
    println!("✓ WASM response code idiom maintained");
    println!("✓ Standard Rust Result<T, E> used");
    println!("✓ Standard Error trait implemented");
    println!("✓ All existing functionality preserved");
    println!("✓ Better error messages with Display trait");
    
    Ok(())
}

// Example function demonstrating error propagation with ? operator
fn example_function_with_error_propagation() -> Result<String> {
    // This would normally call a host function that returns a result code
    let simulated_result_code = -1; // INTERNAL_ERROR
    
    // Use ? operator for clean error propagation
    let _data = match_result_code(simulated_result_code, || "some data".to_string())?;
    
    Ok("This won't be reached due to error above".to_string())
}
