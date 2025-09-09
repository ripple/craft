#![allow(clippy::needless_borrows_for_generic_args)]
use anyhow::Result;
use ci_tools::markdown_tests;
use colored::*;

/// Run markdown code block tests - mirrors the run-markdown job from GitHub Actions
#[tokio::main]
async fn main() -> Result<()> {
    println!(
        "{}",
        "📝 Running markdown code block tests...".cyan().bold()
    );

    // Run markdown tests
    println!("\n{}", "🔧 Running: Markdown tests".yellow());
    if let Err(e) = markdown_tests().await {
        println!("{}", format!("❌ Markdown tests: FAILED - {}", e).red());
        std::process::exit(1);
    } else {
        println!("{}", "✅ Markdown tests: PASSED".green());
    }

    println!(
        "\n{}",
        "🎉 Markdown tests completed successfully!".green().bold()
    );
    Ok(())
}
