use anyhow::{Context, Result};
use colored::*;
use inquire::{Confirm, Select};
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

    // Try to find the WASM file with the exact crate name
    let wasm_file = utils::find_wasm_output(project_dir)?;

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
        // Check if we're in an interactive environment
        use std::io::IsTerminal;
        if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
            // Non-interactive environment (e.g., CI) - skip optimization
            println!(
                "{}",
                "wasm-opt not found. Skipping optimization in non-interactive environment."
                    .yellow()
            );
            return Ok(());
        }

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
        // Check if current directory is a valid WASM project
        if !utils::is_valid_rust_project(&current_dir) {
            println!("{}", "Current directory is not a valid WASM project.".red());
            println!();
            println!("{}", "To use craft, you need to either:".yellow());
            println!(
                "  1. Navigate to a WASM project directory (with cdylib crate-type in Cargo.toml)"
            );
            println!("  2. Create a new WASM project in projects/ or projects/examples/");
            println!("  3. Run 'craft build <project-name>' to specify an existing project");
            println!();
            println!("{}", "A valid WASM project needs:".cyan());
            println!("  • A Cargo.toml file");
            println!("  • crate-type = [\"cdylib\"] in the [lib] section");
            println!();
            println!("{}", "Example:".cyan());
            println!("  cd projects/examples/smart-escrows/notary");
            println!("  craft build");
            anyhow::bail!("Not in a valid WASM project directory");
        }

        println!("Using current directory: {}", current_dir.display());
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
    let current_dir = std::env::current_dir()?;
    let projects = utils::find_wasm_projects(&current_dir);

    if projects.is_empty() {
        println!("{}", "No WASM projects found.".yellow());
        println!("Looking in: projects/examples/");
        return Ok(());
    }

    println!("{}", "Available WASM projects:".cyan());
    for project_path in projects {
        // Display the path relative to current directory for clarity
        let relative_path = project_path
            .strip_prefix(&current_dir)
            .unwrap_or(&project_path);
        println!("  • {}", relative_path.display());
    }
    Ok(())
}

fn list_all_projects() -> Result<Vec<String>> {
    let mut projects = Vec::new();
    let projects_dir = std::env::current_dir()?.join("projects/examples/smart-escrow");

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

    // Show fixtures in wasm_host_simulator directory (only for backward compatibility with escrow)
    let wasm_host_simulator_fixtures = std::env::current_dir()?
        .join("../../../wasm_host_simulator")
        .join("fixtures");
    if wasm_host_simulator_fixtures.exists() {
        println!(
            "\n{}",
            "wasm-host-simulator/fixtures/ (legacy location for escrow):".bold()
        );
        print_tree(&wasm_host_simulator_fixtures, "  ")?;
    }

    // Show fixtures in project directories
    let projects_dir = std::env::current_dir()?.join("projects/examples/smart-escrow");
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
            println!(
                "  • Check test fixtures in wasm_host_simulator/fixtures/<project>/{test_case}/"
            );
            println!("  • Verify your WASM module exports 'allocate' and 'finish' functions");
        }

        anyhow::bail!("Test '{}' failed", test_case);
    }

    Ok(())
}
