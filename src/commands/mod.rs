use anyhow::{Context, Result};
use colored::*;
use inquire::{Confirm, Select};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use crate::config::{BuildMode, Config, OptimizationLevel, WasmTarget};
use crate::utils;

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
        anyhow::bail!("Build failed");
    }

    let target_dir = project_dir.join("target");
    let build_dir = target_dir
        .join(config.wasm_target.to_string())
        .join(config.build_mode.to_string());

    // Get the project name from Cargo.toml instead of directory name
    let cargo_toml_path = cargo_toml.clone(); // Clone the PathBuf to avoid borrowing issues
    let cargo_content = std::fs::read_to_string(&cargo_toml_path)?;
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
            "WASM file not found at expected location: {:?}\nAlternate location checked: {:?}",
            wasm_file,
            alt_wasm_file
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

    // Ask if user wants to export as hex
    if Confirm::new("Would you like to export the WASM as hex (copied to clipboard)?")
        .with_default(false)
        .prompt()?
    {
        copy_wasm_hex_to_clipboard(&wasm_file).await?;
    }

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
        anyhow::bail!("Failed to install Node.js dependencies");
    }

    println!("{}", "Dependencies installed successfully!".green());

    // Run deploy_sample.js with Node.js, passing the hex file
    let output = Command::new("node")
        .arg("reference/js/deploy_sample.js")
        .arg(&hex_file)
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

    println!("{}", "Optimizing WASM module...".cyan());
    utils::optimize_wasm(wasm_path, &opt_level.to_string())?;
    println!("{}", "Optimization complete!".green());
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

pub async fn test(wasm_path: &Path, _function: Option<String>) -> Result<()> {
    println!("{}", "Testing WASM contract...".cyan());

    // Build wasm-host first
    println!("Building wasm-host...");
    let status = Command::new("cargo")
        .args(["build", "--release", "-p", "wasm-host"])
        .status()
        .context("Failed to build wasm-host")?;

    if !status.success() {
        anyhow::bail!("Failed to build wasm-host");
    }

    // Get the path to the wasm-host binary
    let wasm_host_path = std::env::current_dir()?
        .join("target")
        .join("release")
        .join("wasm-host");

    // Select test case
    let test_cases = vec![
        "success (notary account matches)",
        "failure (wrong notary account)",
    ];

    let test_case = Select::new("Select test case:", test_cases).prompt()?;
    let test_case = match test_case {
        "success (notary account matches)" => "success",
        "failure (wrong notary account)" => "failure",
        _ => "success",
    };

    println!("Testing escrow finish condition...");

    // Check if we're running in verbose mode
    let verbose = std::env::var("RUST_LOG")
        .map(|v| v.to_lowercase().contains("debug"))
        .unwrap_or(false);

    let mut args = vec![
        "--wasm-file",
        wasm_path.to_str().unwrap(),
        "--test-case",
        test_case,
    ];

    if verbose {
        args.push("--verbose");
    }

    let output = Command::new(&wasm_host_path)
        .args(&args)
        .output()
        .context("Failed to run wasm-host")?;

    // Print the output
    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stderr).red());
        anyhow::bail!("Test failed");
    }

    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

pub async fn open_explorer() -> Result<()> {
    use open;
    open::that("https://custom.xrpl.org/localhost:6006")?;
    println!(
        "{}",
        "The Explorer should be available at: https://custom.xrpl.org/localhost:6006".blue()
    );

    Ok(())
}
