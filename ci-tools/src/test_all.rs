#![allow(clippy::needless_borrows_for_generic_args)]
use anyhow::Result;
use ci_tools::test_native;
use colored::*;

/// Run all tests - mirrors the test-all functionality from GitHub Actions
#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "🧪 Running all tests...".cyan().bold());

    // Run native tests
    println!("\n{}", "🔧 Running: Native tests".yellow());
    if let Err(e) = test_native().await {
        println!("{}", format!("❌ Native tests: FAILED - {}", e).red());
        std::process::exit(1);
    } else {
        println!("{}", "✅ Native tests: PASSED".green());
    }

    println!(
        "\n{}",
        "🎉 All tests completed successfully!".green().bold()
    );
    Ok(())
}
