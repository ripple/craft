#![allow(clippy::needless_borrows_for_generic_args)]
use anyhow::Result;
use ci_tools::e2e_tests;
use colored::*;

/// Run E2E integration tests - mirrors the e2e-tests job from GitHub Actions
#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "🧪 Running E2E integration tests...".cyan().bold());

    // Run E2E tests
    println!("\n{}", "🔧 Running: E2E tests".yellow());
    if let Err(e) = e2e_tests().await {
        println!("{}", format!("❌ E2E tests: FAILED - {}", e).red());
        std::process::exit(1);
    } else {
        println!("{}", "✅ E2E tests: PASSED".green());
    }

    println!(
        "\n{}",
        "🎉 E2E tests completed successfully!".green().bold()
    );
    Ok(())
}
