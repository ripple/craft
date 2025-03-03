use anyhow::{Context, Result};
use colored::*;
use inquire::{Confirm, Select};
use std::path::{PathBuf, Path};
use std::process::{Command, Output};

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
        
        println!("{}", "\nWarning: Output filename collision detected.".yellow());
        println!("This happens when both a library and binary target share the same name.");
        println!("\nPotential colliding files:");
        println!("Library target: {}", base_path.join(format!("lib{}.wasm", project_dir.file_name().unwrap().to_string_lossy())).display());
        println!("Binary target:  {}", base_path.join(format!("{}.wasm", project_dir.file_name().unwrap().to_string_lossy())).display());
        println!("\nExplanation:");
        println!("- A library target (.lib) is used when your code is meant to be used as a dependency");
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
        println!("\n{}", stderr);
    }

    Ok(())
}

pub async fn build(config: &Config) -> Result<PathBuf> {
    println!("{}", "Building WASM module...".cyan());

    // Check if the WASM target is installed
    let target_str = config.wasm_target.to_string();
    if !utils::check_wasm_target_installed(&target_str) {
        println!("{}", format!("WASM target {} not found. Installing...", target_str).yellow());
        utils::install_wasm_target(&target_str)?;
        println!("{}", format!("Successfully installed {}", target_str).green());
    }

    let cargo_toml = utils::find_cargo_toml(&config.project_path)
        .context("Could not find Cargo.toml in the current directory or its parents")?;
    let project_dir = cargo_toml.parent().unwrap();
    
    let mut args = vec!["build", "--target", &target_str];
    
    if matches!(config.build_mode, BuildMode::Release) {
        args.push("--release");
    }

    println!("{}", "Running cargo build...".cyan());
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
    let wasm_file = target_dir
        .join(config.wasm_target.to_string())
        .join(config.build_mode.to_string())
        .join(project_dir.file_name().unwrap())
        .with_extension("wasm");

    if !wasm_file.exists() {
        anyhow::bail!("WASM file not found at expected location: {:?}", wasm_file);
    }

    println!("{}", "\nBuild completed successfully!".green());
    println!("{}", "\nWASM file location:".cyan());
    println!("{}", wasm_file.display().to_string().white().bold());
    
    let size = std::fs::metadata(&wasm_file)?.len();
    println!("Size: {} bytes", size);

    Ok(wasm_file)
}

pub async fn optimize(wasm_path: &PathBuf, opt_level: &OptimizationLevel) -> Result<()> {
    if !utils::check_wasm_opt_installed() {
        println!("{}", "wasm-opt not found. Would you like to install it?".yellow());
        if Confirm::new("Install wasm-opt?").with_default(true).prompt()? {
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
    let projects = utils::find_wasm_projects(&current_dir);
    
    let project_path = if projects.is_empty() {
        println!("{}", "No WASM projects found in the projects directory.".yellow());
        println!("Using current directory...");
        current_dir
    } else {
        let project_choices: Vec<_> = projects
            .iter()
            .filter_map(|p| utils::get_project_name(p))
            .collect();

        if project_choices.len() == 1 {
            println!("{}", format!("Using project: {}", project_choices[0]).cyan());
            projects[0].clone()
        } else {
            let selected = Select::new("Select WASM project:", project_choices.clone()).prompt()?;
            projects.iter()
                .find(|p| utils::get_project_name(p).as_deref() == Some(&selected))
                .unwrap()
                .clone()
        }
    };

    let targets = vec![
        "wasm32-unknown-unknown (for most blockchain deployments)",
        "wasm32-wasi-preview1 (for WASI compatible environments)",
    ];
    
    let target_idx = Select::new("Select WASM target:", targets).prompt()?;
    let target = match target_idx {
        "wasm32-unknown-unknown (for most blockchain deployments)" => WasmTarget::UnknownUnknown,
        _ => WasmTarget::Wasip1,
    };

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

    let use_wee_alloc = Confirm::new("Use wee_alloc for smaller binary size?")
        .with_default(false)
        .prompt()?;

    Ok(Config {
        wasm_target: target,
        build_mode,
        optimization_level,
        use_wee_alloc,
        project_path,
    })
}

pub async fn export_hex(wasm_path: &PathBuf) -> Result<()> {
    println!("{}", "Converting WASM to hex...".cyan());
    let hex = utils::wasm_to_hex(wasm_path)?;
    utils::copy_to_clipboard(&hex)?;
    println!("{}", "Hex copied to clipboard!".green());
    Ok(())
}

pub async fn setup_wee_alloc(project_path: &PathBuf) -> Result<()> {
    let cargo_toml = utils::find_cargo_toml(project_path)
        .context("Could not find Cargo.toml")?;

    // Read current content
    let mut content = std::fs::read_to_string(&cargo_toml)?;
    
    if !content.contains("wee_alloc") {
        content.push_str("\nwee_alloc = \"0.4.5\"\n");
        std::fs::write(&cargo_toml, content)?;
        
        println!("{}", "Added wee_alloc dependency".green());
        println!("Add this to your lib.rs/main.rs:");
        println!("{}", r#"
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
"#.yellow());
    }

    Ok(())
}

pub async fn test(wasm_path: &PathBuf, function: Option<String>) -> Result<()> {
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

    // Run wasm-host with the appropriate arguments
    // Default function depends on the project being tested
    let function = function.unwrap_or_else(|| {
        if wasm_path.to_string_lossy().contains("json_account_id_compare") {
            "compare_accountID".to_string()
        } else {
            "get_greeting".to_string()
        }
    });
    
    println!("Testing function: {}", function);

    let output = Command::new(&wasm_host_path)
        .args([
            "--wasm-file",  // Changed from --wasm-path to --wasm-file
            wasm_path.to_str().unwrap(),
            "--function",
            &function,
        ])
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