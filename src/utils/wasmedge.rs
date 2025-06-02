use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;
use std::process::Command;

/// Check if WasmEdge is installed and accessible
pub fn is_wasmedge_installed() -> bool {
    // Check if wasmedge command is available
    if Command::new("wasmedge").arg("--version").output().is_ok() {
        return true;
    }
    
    // Check if WasmEdge is installed in the default location
    let home = env::var("HOME").unwrap_or_default();
    let wasmedge_lib_path = PathBuf::from(&home).join(".wasmedge").join("lib");
    wasmedge_lib_path.exists() && wasmedge_lib_path.join("libwasmedge.0.dylib").exists()
}

/// Install WasmEdge using the official installer script
pub async fn install_wasmedge() -> Result<()> {
    println!("Installing WasmEdge...");
    
    // Download and run the WasmEdge installer using sh -c
    let status = Command::new("sh")
        .arg("-c")
        .arg("curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash")
        .status()
        .context("Failed to download WasmEdge installer")?;
    
    if !status.success() {
        anyhow::bail!("Failed to install WasmEdge");
    }
    
    println!("WasmEdge installed successfully!");
    Ok(())
}

/// Get the WasmEdge library path for macOS
pub fn get_wasmedge_lib_path() -> Result<PathBuf> {
    let home = env::var("HOME").context("HOME environment variable not set")?;
    let lib_path = PathBuf::from(home).join(".wasmedge").join("lib");
    
    if !lib_path.exists() {
        anyhow::bail!("WasmEdge library path does not exist: {}", lib_path.display());
    }
    
    Ok(lib_path)
}

/// Configure environment for WasmEdge on macOS
pub fn configure_macos_environment() -> Result<()> {
    // Only configure on macOS
    if !cfg!(target_os = "macos") {
        return Ok(());
    }
    
    let lib_path = get_wasmedge_lib_path()?;
    
    // Get current DYLD_LIBRARY_PATH
    let current_path = env::var("DYLD_LIBRARY_PATH").unwrap_or_default();
    
    // Add WasmEdge lib path if not already present
    let lib_path_str = lib_path.to_string_lossy();
    if !current_path.contains(&*lib_path_str) {
        let new_path = if current_path.is_empty() {
            lib_path_str.to_string()
        } else {
            format!("{}:{}", current_path, lib_path_str)
        };
        
        env::set_var("DYLD_LIBRARY_PATH", new_path);
        println!("Updated DYLD_LIBRARY_PATH to include WasmEdge library path");
    }
    
    Ok(())
}

/// Ensure WasmEdge is installed and configured for the current platform
pub async fn ensure_wasmedge_ready() -> Result<()> {
    if !is_wasmedge_installed() {
        println!("WasmEdge not found. Installing...");
        install_wasmedge().await?;
    }
    
    // Configure environment for macOS
    if cfg!(target_os = "macos") {
        configure_macos_environment()?;
    }
    
    Ok(())
}
