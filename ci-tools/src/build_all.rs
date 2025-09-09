#![allow(clippy::needless_borrows_for_generic_args)]
use anyhow::Result;
use colored::*;
use std::process::Command;

/// Helper function to run a command and show detailed output
fn run_command_with_output(cmd: &mut Command, description: &str) -> Result<()> {
    println!("  🔧 {}", description.cyan());

    let output = cmd.output()?;

    if !output.stdout.is_empty() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("     📤 stdout:");
        for line in stdout.lines() {
            println!("       {}", line);
        }
    }

    if !output.stderr.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if output.status.success() {
            println!("     📝 stderr:");
        } else {
            println!("     ❌ stderr:");
        }
        for line in stderr.lines() {
            if output.status.success() {
                println!("       {}", line.yellow());
            } else {
                println!("       {}", line.red());
            }
        }
    }

    if !output.status.success() {
        anyhow::bail!(
            "{} failed with exit code: {:?}",
            description,
            output.status.code()
        );
    }

    println!("     ✅ {}", "completed successfully".green());
    Ok(())
}

/// Build all workspace projects (native and WASM)
#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "🔧 Building all workspace projects...".cyan().bold());

    // Ensure wasm32 target is installed
    let mut cmd = Command::new("rustup");
    cmd.args(&["target", "add", "wasm32-unknown-unknown"]);
    run_command_with_output(&mut cmd, "Installing wasm32-unknown-unknown target")?;

    // Build native workspace
    let mut cmd = Command::new("cargo");
    cmd.args(&["build", "--workspace"]);
    run_command_with_output(&mut cmd, "Building native workspace")?;

    // Build xrpl-std for WASM
    let mut cmd = Command::new("cargo");
    cmd.args(&[
        "build",
        "-p",
        "xrpl-std",
        "--target",
        "wasm32-unknown-unknown",
    ]);
    run_command_with_output(&mut cmd, "Building xrpl-std for WASM")?;

    // Also run rustc check with warnings as errors
    let mut cmd = Command::new("cargo");
    cmd.args(&[
        "rustc",
        "-p",
        "xrpl-std",
        "--target",
        "wasm32-unknown-unknown",
        "--",
        "-D",
        "warnings",
    ]);
    run_command_with_output(
        &mut cmd,
        "Running rustc check on xrpl-std WASM (warnings as errors)",
    )?;

    // Build WASM projects workspace
    let mut cmd = Command::new("cargo");
    cmd.args(&["build", "--workspace", "--target", "wasm32-unknown-unknown"])
        .current_dir("projects");
    run_command_with_output(&mut cmd, "Building WASM projects workspace")?;

    // Build WASM projects workspace in release mode
    let mut cmd = Command::new("cargo");
    cmd.args(&[
        "build",
        "--workspace",
        "--target",
        "wasm32-unknown-unknown",
        "--release",
    ])
    .current_dir("projects");
    run_command_with_output(&mut cmd, "Building WASM projects workspace (release mode)")?;

    println!(
        "\n{}",
        "🎉 All builds completed successfully!".green().bold()
    );
    Ok(())
}
