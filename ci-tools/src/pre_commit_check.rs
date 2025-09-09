#![allow(clippy::needless_borrows_for_generic_args)]
use anyhow::Result;
use ci_tools::check_precommit;
use colored::*;

/// Run pre-commit checks - mirrors the pre-commit job from GitHub Actions
#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "🔍 Running pre-commit checks...".cyan().bold());

    // Run pre-commit hooks
    println!("\n{}", "🔧 Running: Pre-commit hooks".yellow());
    if let Err(e) = check_precommit().await {
        println!("{}", format!("❌ Pre-commit hooks: FAILED - {}", e).red());
        std::process::exit(1);
    } else {
        println!("{}", "✅ Pre-commit hooks: PASSED".green());
    }

    println!(
        "\n{}",
        "🎉 Pre-commit checks completed successfully!"
            .green()
            .bold()
    );
    Ok(())
}
