#![allow(clippy::needless_borrows_for_generic_args)]
use anyhow::Result;
use ci_tools::{
    build_native, build_wasm_full, check_clippy_all, check_fmt, check_host_functions,
    check_precommit, check_wasm_exports, craft_build_examples, e2e_tests, markdown_tests,
    test_native,
};
use colored::*;

/// Full CI check - runs all CI checks including slower ones
#[tokio::main]
async fn main() -> Result<()> {
    println!(
        "{}",
        "🚀 Running FULL CI checks locally (this may take a while)..."
            .cyan()
            .bold()
    );

    let mut failed_checks = Vec::new();

    // Run checks sequentially
    println!("\n{}", "🔧 Running: Pre-commit hooks".yellow());
    if let Err(e) = check_precommit().await {
        println!("{}", format!("❌ Pre-commit hooks: FAILED - {}", e).red());
        failed_checks.push("Pre-commit hooks");
    } else {
        println!("{}", "✅ Pre-commit hooks: PASSED".green());
    }

    println!("\n{}", "🔧 Running: Rust formatting".yellow());
    if let Err(e) = check_fmt().await {
        println!("{}", format!("❌ Rust formatting: FAILED - {}", e).red());
        failed_checks.push("Rust formatting");
    } else {
        println!("{}", "✅ Rust formatting: PASSED".green());
    }

    println!("\n{}", "🔧 Running: Clippy (All Targets)".yellow());
    if let Err(e) = check_clippy_all().await {
        println!("{}", format!("❌ Clippy (All Targets): FAILED - {}", e).red());
        failed_checks.push("Clippy (All Targets)");
    } else {
        println!("{}", "✅ Clippy (All Targets): PASSED".green());
    }

    println!("\n{}", "🔧 Running: WASM exports".yellow());
    if let Err(e) = check_wasm_exports().await {
        println!("{}", format!("❌ WASM exports: FAILED - {}", e).red());
        failed_checks.push("WASM exports");
    } else {
        println!("{}", "✅ WASM exports: PASSED".green());
    }

    println!("\n{}", "🔧 Running: Host functions audit".yellow());
    if let Err(e) = check_host_functions().await {
        println!(
            "{}",
            format!("❌ Host functions audit: FAILED - {}", e).red()
        );
        failed_checks.push("Host functions audit");
    } else {
        println!("{}", "✅ Host functions audit: PASSED".green());
    }

    println!("\n{}", "🔧 Running: Build native".yellow());
    if let Err(e) = build_native().await {
        println!("{}", format!("❌ Build native: FAILED - {}", e).red());
        failed_checks.push("Build native");
    } else {
        println!("{}", "✅ Build native: PASSED".green());
    }

    println!("\n{}", "🔧 Running: Build WASM".yellow());
    if let Err(e) = build_wasm_full().await {
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

    println!("\n{}", "🔧 Running: Craft build examples".yellow());
    if let Err(e) = craft_build_examples().await {
        println!(
            "{}",
            format!("❌ Craft build examples: FAILED - {}", e).red()
        );
        failed_checks.push("Craft build examples");
    } else {
        println!("{}", "✅ Craft build examples: PASSED".green());
    }

    println!("\n{}", "🔧 Running: Markdown tests".yellow());
    if let Err(e) = markdown_tests().await {
        println!("{}", format!("❌ Markdown tests: FAILED - {}", e).red());
        failed_checks.push("Markdown tests");
    } else {
        println!("{}", "✅ Markdown tests: PASSED".green());
    }

    println!("\n{}", "🔧 Running: E2E integration tests".yellow());
    if let Err(e) = e2e_tests().await {
        println!(
            "{}",
            format!("❌ E2E integration tests: FAILED - {}", e).red()
        );
        failed_checks.push("E2E integration tests");
    } else {
        println!("{}", "✅ E2E integration tests: PASSED".green());
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
