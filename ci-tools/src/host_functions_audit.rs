#![allow(clippy::needless_borrows_for_generic_args)]
use anyhow::Result;
use ci_tools::check_host_functions;
use colored::*;

/// Audit host functions to ensure they match XRPLd - mirrors the host_function_audit job from GitHub Actions
#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "🔍 Auditing host functions...".cyan().bold());

    // Run host functions audit
    println!("\n{}", "🔧 Running: Host functions audit".yellow());
    if let Err(e) = check_host_functions().await {
        println!(
            "{}",
            format!("❌ Host functions audit: FAILED - {}", e).red()
        );
        std::process::exit(1);
    } else {
        println!("{}", "✅ Host functions audit: PASSED".green());
    }

    println!(
        "\n{}",
        "🎉 Host functions audit completed successfully!"
            .green()
            .bold()
    );
    Ok(())
}
