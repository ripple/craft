#![allow(clippy::needless_borrows_for_generic_args)]
use anyhow::Result;
use ci_tools::check_wasm_exports;
use colored::*;

/// Check WASM contract exports - mirrors the wasm exports check from GitHub Actions
#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "🔍 Checking WASM contract exports...".cyan().bold());

    // Check WASM exports
    println!("\n{}", "🔧 Running: WASM exports check".yellow());
    if let Err(e) = check_wasm_exports().await {
        println!("{}", format!("❌ WASM exports check: FAILED - {}", e).red());
        std::process::exit(1);
    } else {
        println!("{}", "✅ WASM exports check: PASSED".green());
    }

    println!(
        "\n{}",
        "🎉 WASM exports check completed successfully!"
            .green()
            .bold()
    );
    Ok(())
}
