use anyhow::{Context, Result};
use colored::*;
use inquire::{Confirm, Select, Text};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use crate::config::{BuildMode, Config, OptimizationLevel, WasmTarget};
use crate::utils;

mod test;
pub use test::TestRunner;

fn handle_build_output(output: &Output, config: &Config, project_dir: &Path) -> Result<()> {
    let _stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Check for filename collision warning
    if stderr.contains("output filename collision") {
        let target_dir = project_dir.join("target");
        let base_path = target_dir
            .join(config.wasm_target.to_string())
            .join(config.build_mode.to_string());

        println!(
            "{}",
            "\nWarning: Output filename collision detected.".yellow()
        );
        println!("This happens when both a library and binary target share the same name.");
        println!("\nPotential colliding files:");
        println!(
            "Library target: {}",
            base_path
                .join(format!(
                    "lib{}.wasm",
                    project_dir.file_name().unwrap().to_string_lossy()
                ))
                .display()
        );
        println!(
            "Binary target:  {}",
            base_path
                .join(format!(
                    "{}.wasm",
                    project_dir.file_name().unwrap().to_string_lossy()
                ))
                .display()
        );
        println!("\nExplanation:");
        println!(
            "- A library target (.lib) is used when your code is meant to be used as a dependency"
        );
        println!("- A binary target (.bin) is used when your code is meant to be an executable");
        println!("When both exist with the same name, Cargo needs to know which one to use.");

        if Confirm::new("Would you like to proceed with the build anyway?")
            .with_default(true)
            .prompt()?
        {
            println!("Proceeding with build...");
        } else {
            anyhow::bail!("Build cancelled by user");
        }
    }

    // Print any other warnings or errors
    if !stderr.is_empty() {
        println!("\n{stderr}");
    }

    Ok(())
}

pub async fn build(config: &Config) -> Result<PathBuf> {
    println!("{}", "Building WASM module...".cyan());

    // Check if the WASM target is installed
    let target_str = config.wasm_target.to_string();
    if !utils::check_wasm_target_installed(&target_str) {
        println!(
            "{}",
            format!("WASM target {target_str} not found. Installing...").yellow()
        );
        utils::install_wasm_target(&target_str)?;
        println!("{}", format!("Successfully installed {target_str}").green());
    }

    let cargo_toml = utils::find_cargo_toml(&config.project_path)
        .context("Could not find Cargo.toml in the current directory or its parents")?;
    let project_dir = cargo_toml.parent().unwrap();

    let mut args = vec!["build", "--target", &target_str];

    if matches!(config.build_mode, BuildMode::Release) {
        args.push("--release");
    }

    println!("{}", "Running cargo build...".cyan());
    println!("args: {args:?}");
    let output = Command::new("cargo")
        .current_dir(project_dir)
        .args(&args)
        .output()
        .context("Failed to execute cargo build")?;

    handle_build_output(&output, config, project_dir)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Check for common build errors and provide helpful suggestions
        if stderr.contains("could not find `Cargo.toml`") {
            anyhow::bail!(
                "Build failed: No Cargo.toml found.\n\n{}",
                "Suggestions:\n  • Make sure you're in a Rust project directory\n  • Run 'craft build' from the workspace root\n  • Check if Cargo.toml exists in the project directory"
            );
        } else if stderr.contains("no targets specified") {
            anyhow::bail!(
                "Build failed: No build targets specified.\n\n{}",
                "Suggestions:\n  • Add a [lib] or [[bin]] section to Cargo.toml\n  • For WASM contracts, you typically need a [lib] section\n  • Example:\n    [lib]\n    crate-type = [\"cdylib\"]"
            );
        } else if stderr.contains("failed to select a version") {
            anyhow::bail!(
                "Build failed: Dependency resolution error.\n\n{}",
                "Suggestions:\n  • Run 'cargo update' to update dependencies\n  • Check for version conflicts in Cargo.toml\n  • Try 'cargo clean' then rebuild"
            );
        } else {
            anyhow::bail!("Build failed. Run with RUST_LOG=debug for more details");
        }
    }

    let target_dir = project_dir.join("target");
    let build_dir = target_dir
        .join(config.wasm_target.to_string())
        .join(config.build_mode.to_string());

    // Get the project name from Cargo.toml instead of directory name
    let cargo_content = std::fs::read_to_string(&cargo_toml)?;
    let name_pattern = regex::Regex::new(r#"name\s*=\s*"([^"]*)""#)?;

    let project_name = if let Some(caps) = name_pattern.captures(&cargo_content) {
        caps.get(1).map(|m| m.as_str().to_string())
    } else {
        // Fall back to directory name if we can't extract from Cargo.toml
        project_dir
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string())
    };

    let project_name = project_name.unwrap_or_else(|| "unknown".to_string());

    // Try to find the WASM file with the exact crate name
    let wasm_file = build_dir.join(&project_name).with_extension("wasm");

    if !wasm_file.exists() {
        // If not found, check if the directory name and crate name differ
        // (which happens with hyphens vs underscores)
        let dir_name = project_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");

        let alt_wasm_file = build_dir.join(dir_name).with_extension("wasm");

        if alt_wasm_file.exists() {
            println!(
                "{}",
                "\nFound WASM file with directory name instead of crate name.".yellow()
            );
            println!(
                "This can happen when directory name contains hyphens but crate name uses underscores."
            );
            return Ok(alt_wasm_file);
        }

        // Also check for lib prefix, which can happen with library crates
        let lib_wasm_file = build_dir
            .join(format!("lib{}", &project_name))
            .with_extension("wasm");
        if lib_wasm_file.exists() {
            return Ok(lib_wasm_file);
        }

        anyhow::bail!(
            "WASM file not found at expected locations:\n  • {:?}\n  • {:?}\n\n{}",
            wasm_file,
            alt_wasm_file,
            "Suggestions:\n  • Make sure the build completed successfully\n  • Check if the crate type is set to 'cdylib' in Cargo.toml\n  • Verify the project name matches the package name\n  • Try running 'craft build' again with --debug flag"
        );
    }

    println!("{}", "\nBuild completed successfully!".green());
    println!("{}", "\nWASM file location:".cyan());
    println!("{}", wasm_file.display().to_string().white().bold());

    let size = std::fs::metadata(&wasm_file)?.len();
    println!("Size: {size} bytes");

    // Calculate and display WASM fingerprint
    let fingerprint = utils::calculate_wasm_fingerprint(&wasm_file)?;
    println!("WASM Fingerprint: {fingerprint}");

    Ok(wasm_file)
}

pub async fn deploy_to_wasm_devnet(wasm_file: &Path) -> Result<()> {
    println!("{}", "Deploying to WASM Devnet...".cyan());

    // Convert WASM to hex and save to file
    let hex = utils::wasm_to_hex(wasm_file)?;
    let hex_file = wasm_file.with_extension("hex");
    std::fs::write(&hex_file, &hex).context("Failed to write hex file")?;

    println!(
        "{}",
        format!("Saved WASM hex to: {}", hex_file.display()).cyan()
    );

    // Always install dependencies
    println!("{}", "Installing required Node.js dependencies...".yellow());

    // Install dependencies in the reference/js directory
    let install_status = Command::new("npm")
        .current_dir("reference/js")
        .arg("install")
        .status()
        .context("Failed to install Node.js dependencies")?;

    if !install_status.success() {
        anyhow::bail!(
            "Failed to install Node.js dependencies.\n\n{}",
            "Suggestions:\n  • Make sure Node.js and npm are installed\n  • Check your internet connection\n  • Try running 'npm install' manually in reference/js/\n  • Install Node.js from https://nodejs.org/"
        );
    }

    println!("{}", "Dependencies installed successfully!".green());

    // Run deploy_sample.js with Node.js, passing the wasm file (not hex)
    let output = Command::new("node")
        .arg("reference/js/deploy_sample.js")
        .arg(wasm_file)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to execute deploy_sample.js")?;

    if !output.success() {
        println!("{}", "Deployment failed!".red());
    } else {
        println!("{}", "Deployment completed successfully!".green());
    }

    Ok(())
}

pub async fn copy_wasm_hex_to_clipboard(wasm_file: &Path) -> Result<()> {
    let hex = utils::wasm_to_hex(wasm_file)?;
    utils::copy_to_clipboard(&hex)?;
    println!("{}", "WASM hex copied to clipboard!".green());
    Ok(())
}

pub async fn optimize(wasm_path: &Path, opt_level: &OptimizationLevel) -> Result<()> {
    if !utils::check_wasm_opt_installed() {
        println!(
            "{}",
            "wasm-opt not found. Would you like to install it?".yellow()
        );
        if Confirm::new("Install wasm-opt?")
            .with_default(true)
            .prompt()?
        {
            utils::install_wasm_opt()?;
        } else {
            println!("Skipping optimization...");
            return Ok(());
        }
    }

    use std::fs;
    let before = fs::metadata(wasm_path).map(|m| m.len()).unwrap_or(0);

    println!(
        "{}",
        format!("Optimizing WASM module (level {opt_level})...").cyan()
    );

    utils::optimize_wasm(wasm_path, &opt_level.to_string())?;

    let after = fs::metadata(wasm_path).map(|m| m.len()).unwrap_or(0);
    let saved: i128 = before as i128 - after as i128;
    let saved_pct: f64 = if before > 0 {
        (saved as f64 / before as f64) * 100.0
    } else {
        0.0
    };

    println!(
        "{}",
        format!(
            "Optimization complete! Size: {} → {} bytes (saved {} bytes, {:.1}%)",
            before,
            after,
            saved.max(0),
            saved_pct.max(0.0)
        )
        .green()
    );
    Ok(())
}

pub async fn configure() -> Result<Config> {
    println!("{}", "Configuring WASM build settings...".cyan());

    // Find all WASM projects
    let current_dir = std::env::current_dir()?;
    let mut projects = utils::find_wasm_projects(&current_dir);

    let project_path = if projects.is_empty() {
        println!(
            "{}",
            "No WASM projects found in the projects directory.".yellow()
        );
        println!("Using current directory...");
        current_dir
    } else {
        let project_choices: Vec<_> = projects
            .iter()
            .filter_map(|p| utils::get_project_name(p))
            .collect();

        if project_choices.len() == 1 {
            println!(
                "{}",
                format!("Using project: {}", project_choices[0]).cyan()
            );
            let validated_path = utils::validate_project_name(&projects[0])?;

            // If the path was changed (folder was renamed), update our list
            if validated_path != projects[0] {
                projects = utils::find_wasm_projects(&current_dir);
                // If the folder was renamed and we can't find it anymore, use the validated path
                if !projects.contains(&validated_path) {
                    println!(
                        "{}",
                        format!("Using renamed project at: {}", validated_path.display()).cyan()
                    );
                    validated_path
                } else {
                    validated_path
                }
            } else {
                validated_path
            }
        } else {
            let selected = Select::new("Select WASM project:", project_choices.clone()).prompt()?;
            let selected_idx = project_choices.iter().position(|p| p == &selected).unwrap();
            let selected_path = projects[selected_idx].clone();

            let validated_path = utils::validate_project_name(&selected_path)?;

            // If the path was changed (folder was renamed), update our list
            if validated_path != selected_path {
                projects = utils::find_wasm_projects(&current_dir);
                // If the folder was renamed and we can't find it anymore, use the validated path
                if !projects.contains(&validated_path) {
                    println!(
                        "{}",
                        format!("Using renamed project at: {}", validated_path.display()).cyan()
                    );
                    validated_path
                } else {
                    validated_path
                }
            } else {
                validated_path
            }
        }
    };

    // Always use wasm32-unknown-unknown target
    let target = WasmTarget::UnknownUnknown;

    let build_modes = vec![
        "Release (optimized, no debug info)",
        "Debug (includes debug info)",
    ];

    let build_mode_idx = Select::new("Select build mode:", build_modes).prompt()?;
    let build_mode = match build_mode_idx {
        "Release (optimized, no debug info)" => BuildMode::Release,
        _ => BuildMode::Debug,
    };

    let optimization_levels = vec![
        "None (no optimization)",
        "Small (-Os: optimize for size)",
        "Aggressive (-Oz: optimize aggressively for size)",
    ];

    let opt_idx = Select::new("Select optimization level:", optimization_levels).prompt()?;
    let optimization_level = match opt_idx {
        "None (no optimization)" => OptimizationLevel::None,
        "Small (-Os: optimize for size)" => OptimizationLevel::Small,
        _ => OptimizationLevel::Aggressive,
    };

    Ok(Config {
        wasm_target: target,
        build_mode,
        optimization_level,
        project_path,
    })
}

pub async fn test(wasm_path: &Path, function: Option<String>) -> Result<()> {
    println!("{}", "Testing WASM contract...".cyan());

    // Extract project name from wasm path
    let project_name = wasm_path
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    // Use the new TestRunner for a better interface
    let runner = TestRunner::new(wasm_path, project_name).verbose(false);

    // Interactive test case selection
    let test_cases = vec![
        "success (notary account matches)",
        "failure (wrong notary account)",
        "Run all test cases",
    ];

    let selection = Select::new("Select test case:", test_cases).prompt()?;

    match selection {
        "Run all test cases" => {
            let runner = runner.verbose(true);
            let results = runner.run_all_tests(project_name)?;

            if results.iter().any(|r| !r.success) {
                anyhow::bail!("Some tests failed");
            }
        }
        _ => {
            let test_case = match selection {
                "success (notary account matches)" => "success",
                "failure (wrong notary account)" => "failure",
                _ => "success",
            };

            let result = runner.run_test(test_case, function.as_deref())?;

            println!("{}", result.stdout);

            if !result.success {
                println!("{}", result.stderr.red());

                if let Some(desc) = result.error_description() {
                    println!();
                    println!("{}: {}", "Error".red().bold(), desc);
                }

                anyhow::bail!("Test failed");
            }
        }
    }

    Ok(())
}

pub async fn open_explorer() -> Result<()> {
    open::that("https://custom.xrpl.org/localhost:6006")?;
    println!(
        "{}",
        "The Explorer should be available at: https://custom.xrpl.org/localhost:6006".blue()
    );

    Ok(())
}

// New helper functions for improved craft commands

pub fn list_projects() -> Result<()> {
    let projects_dir = std::env::current_dir()?.join("projects");
    if !projects_dir.exists() {
        println!("{}", "No projects directory found.".yellow());
        return Ok(());
    }

    println!("{}", "Available projects:".cyan());
    for entry in fs::read_dir(projects_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir()
            && let Some(name) = path.file_name()
        {
            println!("  • {}", name.to_string_lossy());
        }
    }
    Ok(())
}

fn list_all_projects() -> Result<Vec<String>> {
    let mut projects = Vec::new();
    let projects_dir = std::env::current_dir()?.join("projects");

    if projects_dir.exists() {
        for entry in fs::read_dir(projects_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir()
                && let Some(name) = path.file_name()
            {
                projects.push(name.to_string_lossy().to_string());
            }
        }
    }

    Ok(projects)
}

pub fn list_test_cases(project: Option<&str>) -> Result<()> {
    if let Some(proj) = project {
        // List test cases for specific project
        let test_cases = discover_test_cases(proj)?;

        if !test_cases.is_empty() {
            println!("{}", format!("Test cases for {proj}:").cyan());
            for test_case in test_cases {
                println!("  • {test_case}");
            }
        } else {
            println!(
                "{}",
                format!("No test cases found for project: {proj}").yellow()
            );
        }
    } else {
        // List all test cases
        println!("{}", "Available test cases by project:".cyan());

        // Get all projects
        let projects = list_all_projects()?;

        for project in projects {
            let test_cases = discover_test_cases(&project)?;
            if !test_cases.is_empty() {
                println!("\n{}:", project.bold());
                for test_case in test_cases {
                    println!("  • {test_case}");
                }
            }
        }
    }
    Ok(())
}

pub fn list_all_tests() -> Result<()> {
    list_test_cases(None)
}

pub fn list_fixtures() -> Result<()> {
    println!("{}", "Test fixtures structure:".cyan());
    println!(
        "{}",
        "Convention: fixtures should be in projects/<project>/fixtures/<test_case>/".italic()
    );

    fn print_tree(dir: &Path, prefix: &str) -> Result<()> {
        for (i, entry) in fs::read_dir(dir)?.enumerate() {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name();

            let is_last = fs::read_dir(dir)?.count() == i + 1;
            let connector = if is_last { "└── " } else { "├── " };

            println!("{}{}{}", prefix, connector, name.to_string_lossy());

            if path.is_dir() && !name.to_str().unwrap().starts_with('.') {
                let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
                print_tree(&path, &new_prefix)?;
            }
        }
        Ok(())
    }

    // Show fixtures in wasm-host directory (only for backward compatibility with escrow)
    let wasm_host_fixtures = std::env::current_dir()?.join("wasm-host").join("fixtures");
    if wasm_host_fixtures.exists() {
        println!(
            "\n{}",
            "wasm-host/fixtures/ (legacy location for escrow):".bold()
        );
        print_tree(&wasm_host_fixtures, "  ")?;
    }

    // Show fixtures in project directories
    let projects_dir = std::env::current_dir()?.join("projects");
    if projects_dir.exists() {
        for entry in fs::read_dir(projects_dir)? {
            let entry = entry?;
            let project_path = entry.path();
            if project_path.is_dir() {
                let fixtures_path = project_path.join("fixtures");
                if fixtures_path.exists()
                    && let Some(project_name) = project_path.file_name()
                {
                    println!(
                        "\n{}",
                        format!("projects/{}/fixtures/:", project_name.to_string_lossy()).bold()
                    );
                    print_tree(&fixtures_path, "  ")?;
                }
            }
        }
    }

    Ok(())
}

pub fn discover_test_cases(project: &str) -> Result<Vec<String>> {
    // Convention: fixtures must be in projects/<project>/fixtures/<test_case>/
    let fixtures_dir = std::env::current_dir()?
        .join("projects")
        .join(project)
        .join("fixtures");

    let mut test_cases = Vec::new();

    if fixtures_dir.exists() {
        for entry in fs::read_dir(fixtures_dir)? {
            let entry = entry?;
            if entry.path().is_dir()
                && let Some(name) = entry.path().file_name()
            {
                test_cases.push(name.to_string_lossy().to_string());
            }
        }
    }

    if test_cases.is_empty() {
        // Fall back to standard test cases
        test_cases.push("success".to_string());
    }

    Ok(test_cases)
}

pub async fn build_with_args(config: &Config, cargo_args: &[String]) -> Result<PathBuf> {
    println!("{}", "Building WASM module...".cyan());

    // Check if the WASM target is installed
    let target_str = config.wasm_target.to_string();
    if !utils::check_wasm_target_installed(&target_str) {
        println!(
            "{}",
            format!("WASM target {target_str} not found. Installing...").yellow()
        );
        utils::install_wasm_target(&target_str)?;
        println!("{}", format!("Successfully installed {target_str}").green());
    }

    let cargo_toml = utils::find_cargo_toml(&config.project_path)
        .context("Could not find Cargo.toml in the current directory or its parents")?;
    let project_dir = cargo_toml.parent().unwrap();

    let mut args = vec!["build", "--target", &target_str];
    if matches!(config.build_mode, BuildMode::Release) {
        args.push("--release");
    }

    // Add any additional cargo args
    for arg in cargo_args {
        args.push(arg);
    }

    println!("{}", "Running cargo build...".cyan());
    println!("args: {args:?}");

    let output = Command::new("cargo")
        .current_dir(project_dir)
        .args(&args)
        .output()
        .context("Failed to execute cargo build")?;

    handle_build_output(&output, config, project_dir)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Check for common build errors and provide helpful suggestions
        if stderr.contains("could not find `Cargo.toml`") {
            anyhow::bail!(
                "Build failed: No Cargo.toml found.\n\n{}",
                "Suggestions:\n  • Make sure you're in a Rust project directory\n  • Run 'craft build' from the workspace root\n  • Check if Cargo.toml exists in the project directory"
            );
        } else if stderr.contains("no targets specified") {
            anyhow::bail!(
                "Build failed: No build targets specified.\n\n{}",
                "Suggestions:\n  • Add a [lib] or [[bin]] section to Cargo.toml\n  • For WASM contracts, you typically need a [lib] section\n  • Example:\n    [lib]\n    crate-type = [\"cdylib\"]"
            );
        } else if stderr.contains("failed to select a version") {
            anyhow::bail!(
                "Build failed: Dependency resolution error.\n\n{}",
                "Suggestions:\n  • Run 'cargo update' to update dependencies\n  • Check for version conflicts in Cargo.toml\n  • Try 'cargo clean' then rebuild"
            );
        } else {
            anyhow::bail!("Build failed. Run with RUST_LOG=debug for more details");
        }
    }

    let wasm_path = utils::find_wasm_output(&config.project_path)?;
    println!(
        "{}",
        format!("Build successful! Output: {}", wasm_path.display()).green()
    );

    Ok(wasm_path)
}

pub fn run_test(
    wasm_path: &Path,
    test_case: &str,
    function: Option<&str>,
    verbose: bool,
    _non_interactive: bool,
) -> Result<()> {
    // Extract project name from wasm path
    let project_name = wasm_path
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    // Use the new TestRunner for consistent interface
    let runner = TestRunner::new(wasm_path, project_name).verbose(verbose);
    let result = runner.run_test(test_case, function)?;

    // Print output
    println!("{}", result.stdout);

    if !result.success {
        println!("{}", result.stderr.red());

        // Special handling for "failure" test cases - they're expected to fail
        if test_case == "failure" {
            return Ok(());
        }

        if let Some(desc) = result.error_description() {
            println!();
            println!("{}: {}", "Error".red().bold(), desc);
            println!();
            println!("{}", "Debug tips:".yellow());
            println!("  • Run with --verbose to see detailed trace output");
            println!("  • Check test fixtures in wasm-host/fixtures/<project>/{test_case}/");
            println!("  • Verify your WASM module exports 'allocate' and 'finish' functions");
        }

        anyhow::bail!("Test '{}' failed", test_case);
    }

    Ok(())
}

// ==========================
// Project scaffolding (init)
// ==========================

fn compute_xrpl_std_dependency(_project_dir: &Path) -> String {
    // If running inside the craft monorepo where `xrpl-std/` exists at the repo root,
    // prefer a path dependency. Otherwise, fall back to crates.io (0.5 series).
    // Assumes projects are created under <cwd>/projects/<name> when using craft.
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let repo_root_has_xrpl_std = cwd.join("xrpl-std").exists();

    if repo_root_has_xrpl_std {
        // project_dir is <cwd>/projects/<name>; relative path to xrpl-std is ../../xrpl-std
        // Use a simple relative path without extra computation to avoid pathdiff dependency
        return "xrpl-std = { path = \"../../xrpl-std\" }".to_string();
    }

    // crates.io fallback (allow semver compatible with 0.5)
    "xrpl-std = \"0.5\"".to_string()
}

fn generate_cargo_toml(crate_name: &str, use_path_dep: &str) -> String {
    format!(
        r#"[package]
name = "{crate_name}"
version = "0.0.1"
edition = "2024"
description = "XRPL WASM smart contract"
license = "ISC"

[workspace]

[lib]
crate-type = ["cdylib"]

[profile.release]
opt-level = "s"
lto = true
codegen-units = 1
panic = "abort"

[profile.dev]
panic = "abort"

[dependencies]
{use_path_dep}
"#
    )
}

fn generate_empty_lib_rs() -> String {
    r#"#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_std::host::trace;

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let _ = trace("Hello from craft! This is an empty template.");
    1
}
"#
    .to_string()
}

fn generate_sample_lib_rs() -> String {
    r#"#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_std::host::trace;
use xrpl_std::core::types::transaction_type::TransactionType;
use xrpl_std::core::current_tx::escrow_finish::{get_current_escrow_finish, EscrowFinish};

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let _ = trace("Sample project: starting execution");

    // Access the current transaction and assert it's an EscrowFinish
    let tx: EscrowFinish = get_current_escrow_finish();
    let tx_type = tx.get_transaction_type().unwrap();
    if !matches!(tx_type, TransactionType::EscrowFinish) {
        let _ = trace("Not an EscrowFinish transaction");
        return -1;
    }

    let _ = trace("Sample project: done");
    1
}
"#
    .to_string()
}

fn generate_readme_md(project_name: &str, _crate_name: &str, used_crates_io: bool) -> String {
    let dep_note = if used_crates_io {
        "This project depends on `xrpl-std` from crates.io (0.5 series)."
    } else {
        "This project depends on the local `xrpl-std` via a path dependency (monorepo layout)."
    };

    format!(
        r#"# {project_name}

XRPL WASM smart contract scaffolded by `craft`.

{dep_note}

## Quickstart

- Build: `craft build {project_name}`
- Test (interactive): `craft test {project_name}`
- Export hex: `craft export-hex` (via interactive flow)

## Naming conventions (FYI)

- Folder name may be `kebab-case`, but Rust crate names should be `snake_case`.
- Keep folder and crate names aligned to avoid confusing build artifacts.
- `craft` will suggest fixes if it detects mismatches.

## Notes

- Target: `wasm32-unknown-unknown`
- Library crate with `[lib] crate-type = ["cdylib"]`
- The entrypoint function is `finish()` and must return an `i32`.
"#
    )
}

fn write_file(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)?;
    Ok(())
}

pub async fn init() -> Result<()> {
    println!("{}", "Create a new XRPL WASM project".cyan());

    let project_name = Text::new("Project name (kebab-case)")
        .with_help_message("Example: my-escrow-contract")
        .with_placeholder("my-project")
        .prompt()?;

    // Basic validation
    if project_name.trim().is_empty() {
        anyhow::bail!("Project name cannot be empty");
    }

    let template_choice =
        Select::new("Choose a template", vec!["Sample project", "Empty project"]).prompt()?;

    // Determine locations
    let cwd = std::env::current_dir()?;
    let projects_dir = cwd.join("projects");
    if !projects_dir.exists() {
        fs::create_dir_all(&projects_dir)?;
    }
    let project_dir = projects_dir.join(&project_name);
    if project_dir.exists() {
        anyhow::bail!("Directory already exists: {}", project_dir.display());
    }
    fs::create_dir_all(project_dir.join("src"))?;

    // Derive crate name (snake_case)
    let crate_name = project_name.replace('-', "_");

    // Dependency string and fallback detection
    let dep_str = compute_xrpl_std_dependency(&project_dir);
    let used_crates_io = dep_str.contains('"'); // crude check: contains version string quotes

    // Generate files
    let cargo_toml = generate_cargo_toml(&crate_name, &dep_str);
    let lib_rs = match template_choice {
        "Sample project" => generate_sample_lib_rs(),
        _ => generate_empty_lib_rs(),
    };
    let readme_md = generate_readme_md(&project_name, &crate_name, used_crates_io);
    let gitignore = "/target\n**/*.opt.wasm\n";

    write_file(&project_dir.join("Cargo.toml"), &cargo_toml)?;
    write_file(&project_dir.join("src/lib.rs"), &lib_rs)?;
    write_file(&project_dir.join("README.md"), &readme_md)?;
    write_file(&project_dir.join(".gitignore"), gitignore)?;

    println!("{}", "\nProject created successfully!".green());
    println!(
        "{}",
        format!("Location: {}", project_dir.display())
            .white()
            .bold()
    );
    println!("\nNext steps:");
    println!("  - craft build {project_name}");
    println!("  - craft test {project_name}");

    Ok(())
}
