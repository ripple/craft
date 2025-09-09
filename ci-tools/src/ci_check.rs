#![allow(clippy::needless_borrows_for_generic_args)]
use anyhow::Result;
use ci_tools::{
    build_native, build_wasm, check_clippy_all, check_fmt, check_wasm_exports, test_native,
};
use colored::*;

/// Quick CI check - runs the most important checks locally
#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "🚀 Running quick CI checks locally...".cyan().bold());

    let mut failed_checks = Vec::new();

    // Run checks sequentially
    println!("\n{}", "🔧 Running: Rust formatting".yellow());
    if let Err(e) = check_fmt().await {
        println!("{}", format!("❌ Rust formatting: FAILED - {}", e).red());
        failed_checks.push("Rust formatting");
    } else {
        println!("{}", "✅ Rust formatting: PASSED".green());
    }

    println!("\n{}", "🔧 Running: Clippy (All Targets)".yellow());
    if let Err(e) = check_clippy_all().await {
        println!(
            "{}",
            format!("❌ Clippy (All Targets): FAILED - {}", e).red()
        );
        failed_checks.push("Clippy (All Targets)");
    } else {
        println!("{}", "✅ Clippy (native): PASSED".green());
    }

    println!("\n{}", "🔧 Running: WASM exports".yellow());
    if let Err(e) = check_wasm_exports().await {
        println!("{}", format!("❌ WASM exports: FAILED - {}", e).red());
        failed_checks.push("WASM exports");
    } else {
        println!("{}", "✅ WASM exports: PASSED".green());
    }

    println!("\n{}", "🔧 Running: Build native".yellow());
    if let Err(e) = build_native().await {
        println!("{}", format!("❌ Build native: FAILED - {}", e).red());
        failed_checks.push("Build native");
    } else {
        println!("{}", "✅ Build native: PASSED".green());
    }

    println!("\n{}", "🔧 Running: Build WASM".yellow());
    if let Err(e) = build_wasm().await {
        println!("{}", format!("❌ Build WASM: FAILED - {}", e).red());
        failed_checks.push("Build WASM");
    } else {
        println!("{}", "✅ Build WASM: PASSED".green());
    }

    println!("\n{}", "🔧 Running: Test native".yellow());
    if let Err(e) = test_native().await {
        println!("{}", format!("❌ Test native: FAILED - {}", e).red());
        failed_checks.push("Test native");
    } else {
        println!("{}", "✅ Test native: PASSED".green());
    }

    if failed_checks.is_empty() {
        println!("\n{}", "🎉 All CI checks passed!".green().bold());
        Ok(())
    } else {
        println!(
            "\n{}",
            format!("💥 {} checks failed:", failed_checks.len())
                .red()
                .bold()
        );
        for check in failed_checks {
            println!("  - {}", check.red());
        }
        anyhow::bail!("CI checks failed");
    }
}
