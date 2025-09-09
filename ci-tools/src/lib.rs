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

/// Check Rust formatting
pub async fn check_fmt() -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.args(&["fmt", "--all", "--", "--check"]);

    run_command_with_output(&mut cmd, "Checking Rust formatting")
}

/// Check Clippy on native targets
pub async fn check_clippy_all() -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.args(&[
        "clippy",
        "--workspace",
        "--all-targets",
        "--all-features",
        "--",
        "-Dclippy::all",
    ]);

    run_command_with_output(&mut cmd, "Running Clippy on all targets")
}

/// Check that all WASM projects export the required finish function
pub async fn check_wasm_exports() -> Result<()> {
    use std::fs;
    use walkdir::WalkDir;

    println!("  🔧 {}", "Checking WASM exports".cyan());

    let mut checked_files = 0;
    for entry in WalkDir::new("./projects")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_name() == "lib.rs"
            && entry.path().parent().unwrap().file_name() == Some(std::ffi::OsStr::new("src"))
        {
            println!("     📁 Checking: {}", entry.path().display());
            let content = fs::read_to_string(entry.path())?;
            if !content.contains("finish() -> i32") {
                println!("     ❌ Missing required finish() -> i32 export");
                anyhow::bail!(
                    "Missing required finish() -> i32 export in {:?}",
                    entry.path()
                );
            } else {
                println!("     ✅ Found finish() -> i32 export");
            }
            checked_files += 1;
        }
    }

    println!(
        "     ✅ {} files checked successfully",
        checked_files.to_string().green()
    );
    Ok(())
}

/// Build native workspace
pub async fn build_native() -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.args(&["build", "--workspace"]);

    run_command_with_output(&mut cmd, "Building native workspace")
}

/// Build WASM targets
pub async fn build_wasm() -> Result<()> {
    // Ensure wasm32 target is installed
    let mut cmd = Command::new("rustup");
    cmd.args(&["target", "add", "wasm32-unknown-unknown"]);
    run_command_with_output(&mut cmd, "Installing wasm32-unknown-unknown target")?;

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

    // Build projects workspace for WASM
    let mut cmd = Command::new("cargo");
    cmd.args(&["build", "--workspace", "--target", "wasm32-unknown-unknown"])
        .current_dir("projects");
    run_command_with_output(&mut cmd, "Building projects workspace for WASM")
}

/// Build WASM targets including release mode (for full CI)
pub async fn build_wasm_full() -> Result<()> {
    // First do the regular WASM build
    build_wasm().await?;

    // Build release version too
    let mut cmd = Command::new("cargo");
    cmd.args(&[
        "build",
        "--workspace",
        "--target",
        "wasm32-unknown-unknown",
        "--release",
    ])
    .current_dir("projects");
    run_command_with_output(
        &mut cmd,
        "Building projects workspace for WASM (release mode)",
    )
}

/// Run native tests
pub async fn test_native() -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.args(&["test", "--workspace"]);

    run_command_with_output(&mut cmd, "Running native tests")
}

/// Check pre-commit hooks
pub async fn check_precommit() -> Result<()> {
    // Check if pre-commit is installed
    println!("  🔧 {}", "Checking pre-commit availability".cyan());
    let output = Command::new("pre-commit").args(&["--version"]).output();

    match output {
        Ok(output) if output.status.success() => {
            println!("     ✅ pre-commit is available");
            // Run pre-commit hooks
            let mut cmd = Command::new("pre-commit");
            cmd.args(&["run", "--all-files"]);
            run_command_with_output(&mut cmd, "Running pre-commit hooks")
        }
        _ => {
            println!(
                "{}",
                "     ⚠️  Pre-commit not installed, skipping...".yellow()
            );
            Ok(())
        }
    }
}

/// Audit host functions to ensure they match the XRPLd host functions
pub async fn check_host_functions() -> Result<()> {
    // Check if Node.js is available
    println!("  🔧 {}", "Checking Node.js availability".cyan());
    let output = Command::new("node").args(&["--version"]).output();

    match output {
        Ok(output) if output.status.success() => {
            println!("     ✅ Node.js is available");
            let mut cmd = Command::new("node");
            cmd.args(&[
                "tools/compareHostFunctions.js",
                "https://github.com/XRPLF/rippled/tree/ripple/smart-escrow",
            ]);

            match run_command_with_output(&mut cmd, "Running host functions audit") {
                Ok(_) => Ok(()),
                Err(_) => {
                    println!(
                        "{}",
                        "     ⚠️  Host functions audit failed (this is not required for PRs)"
                            .yellow()
                    );
                    Ok(())
                }
            }
        }
        _ => {
            println!(
                "{}",
                "     ⚠️  Node.js not installed, skipping host functions audit...".yellow()
            );
            Ok(())
        }
    }
}

/// Build examples with craft
pub async fn craft_build_examples() -> Result<()> {
    // Install craft
    let mut cmd = Command::new("cargo");
    cmd.args(&["install", "--path", "craft", "--force"]);
    run_command_with_output(&mut cmd, "Installing craft tool")?;

    // Build examples with craft
    use walkdir::WalkDir;

    println!("  🔧 {}", "Building examples with craft".cyan());
    let mut built_examples = 0;

    for entry in WalkDir::new("./projects/examples")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_name() == "Cargo.toml" {
            let dir = entry.path().parent().unwrap();
            let contract_name = dir.file_name().unwrap().to_string_lossy();

            if dir.join("fixtures").exists() {
                println!("     📦 Building example: {}", contract_name.cyan());
                let mut cmd = Command::new("craft");
                cmd.args(&["build", &contract_name, "-r", "-O", "aggressive"]);

                run_command_with_output(
                    &mut cmd,
                    &format!("Building {} with craft", contract_name),
                )?;
                built_examples += 1;
            } else {
                println!(
                    "     ⏭️  Skipping {} (no fixtures directory)",
                    contract_name.yellow()
                );
            }
        }
    }

    println!(
        "     ✅ {} examples built successfully",
        built_examples.to_string().green()
    );
    Ok(())
}

/// Run markdown tests
pub async fn markdown_tests() -> Result<()> {
    use regex::Regex;
    use std::fs;
    use walkdir::WalkDir;

    println!("  🔧 {}", "Running markdown tests".cyan());
    let bash_block_regex = Regex::new(r"(?s)```bash\n(.*?)\n```")?;
    let mut tested_files = 0;
    let mut executed_blocks = 0;

    // Define the directories to search for markdown files
    let search_dirs = vec![
        "projects",  // projects and all subfolders
        ".",         // root craft folder (but only direct files, not subdirs)
        "examples",  // eventual root examples folder
        "its",       // eventual root its folder
    ];

    for search_dir in search_dirs {
        let search_path = std::path::Path::new(search_dir);

        // Skip if directory doesn't exist
        if !search_path.exists() {
            continue;
        }

        // For root directory, only check direct markdown files, not subdirectories
        let walker = if search_dir == "." {
            WalkDir::new(search_path).max_depth(1)
        } else {
            WalkDir::new(search_path)
        };

        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().and_then(|s| s.to_str()) == Some("md")
                && !entry.path().to_string_lossy().contains("target")
            {
                let content = fs::read_to_string(entry.path())?;
                let md_dir = entry.path().parent().unwrap();
                let mut blocks_in_file = 0;

                for captures in bash_block_regex.captures_iter(&content) {
                    if let Some(bash_code) = captures.get(1) {
                        println!("     📄 Testing bash block in: {}", entry.path().display());
                        println!(
                            "     🔧 Executing: {}",
                            bash_code.as_str().lines().next().unwrap_or("").cyan()
                        );

                        let mut cmd = Command::new("bash");
                        cmd.args(&["-c", bash_code.as_str()]).current_dir(md_dir);

                        run_command_with_output(
                            &mut cmd,
                            &format!("Running bash block in {}", entry.path().display()),
                        )?;
                        blocks_in_file += 1;
                        executed_blocks += 1;
                    }
                }

                if blocks_in_file > 0 {
                    tested_files += 1;
                    println!(
                        "     ✅ {} bash blocks tested in {}",
                        blocks_in_file,
                        entry.path().display()
                    );
                }
            }
        }
    }

    println!(
        "     ✅ {} bash blocks in {} files tested successfully",
        executed_blocks.to_string().green(),
        tested_files.to_string().green()
    );
    Ok(())
}

/// Run E2E integration tests
pub async fn e2e_tests() -> Result<()> {
    // Build all projects first (includes both debug and release WASM builds)
    let mut cmd = Command::new("cargo");
    cmd.args(&["build-all"]);
    run_command_with_output(&mut cmd, "Building all projects for E2E tests")?;

    // Run all tests
    let mut cmd = Command::new("cargo");
    cmd.args(&["test-all"]);
    run_command_with_output(&mut cmd, "Running all tests")?;

    // Run integration tests
    use walkdir::WalkDir;

    println!("  🔧 {}", "Running E2E integration tests".cyan());
    let mut tested_projects = 0;

    for entry in WalkDir::new("./projects")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_name() == "Cargo.toml" {
            let dir = entry.path().parent().unwrap();
            let contract_name = dir.file_name().unwrap().to_string_lossy();

            if dir.join("fixtures").exists() {
                println!("     🧪 Testing project: {}", contract_name.cyan());
                let mut cmd = Command::new("cargo");
                cmd.args(&[
                    "run",
                    "--package",
                    "wasm-host",
                    "--bin",
                    "wasm-host",
                    "--",
                    "-p",
                    &contract_name,
                    "--dir",
                    &dir.to_string_lossy(),
                ]);

                run_command_with_output(
                    &mut cmd,
                    &format!("Running E2E test for {}", contract_name),
                )?;
                tested_projects += 1;
            } else {
                println!(
                    "     ⏭️  Skipping {} (no fixtures directory)",
                    contract_name.yellow()
                );
            }
        }
    }

    println!(
        "     ✅ {} projects tested successfully",
        tested_projects.to_string().green()
    );
    Ok(())
}
