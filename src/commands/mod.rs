use anyhow::{Context, Result};
use colored::*;
use inquire::{Confirm, Select};
use std::path::PathBuf;
use std::process::Command;

use crate::config::{BuildMode, Config, OptimizationLevel, WasmTarget};
use crate::utils;

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
    let status = Command::new("cargo")
        .current_dir(project_dir)
        .args(&args)
        .status()
        .context("Failed to execute cargo build")?;

    if !status.success() {
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

    println!("{}", "Build completed successfully!".green());
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
        project_path: std::env::current_dir()?,
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