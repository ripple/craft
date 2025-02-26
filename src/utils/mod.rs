use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

pub fn find_cargo_toml(start_path: &Path) -> Option<PathBuf> {
    WalkDir::new(start_path)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|e| e.file_name() == "Cargo.toml")
        .map(|e| e.path().to_path_buf())
}

pub fn check_wasm_target_installed(target: &str) -> bool {
    let output = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output();

    match output {
        Ok(output) => {
            let output = String::from_utf8_lossy(&output.stdout);
            output.contains(target)
        }
        Err(_) => false,
    }
}

pub fn install_wasm_target(target: &str) -> Result<()> {
    Command::new("rustup")
        .args(["target", "add", target])
        .status()
        .context(format!("Failed to install WASM target: {}", target))?;
    Ok(())
}

pub fn check_wasm_opt_installed() -> bool {
    which::which("wasm-opt").is_ok()
}

pub fn install_wasm_opt() -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        Command::new("brew")
            .args(["install", "binaryen"])
            .status()
            .context("Failed to install wasm-opt via Homebrew")?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("sudo")
            .args(["apt-get", "install", "-y", "binaryen"])
            .status()
            .context("Failed to install wasm-opt via apt")?;
    }

    Ok(())
}

pub fn optimize_wasm(wasm_path: &Path, opt_level: &str) -> Result<()> {
    let output_path = wasm_path.with_extension("opt.wasm");
    Command::new("wasm-opt")
        .args([
            wasm_path.to_str().unwrap(),
            opt_level,
            "-o",
            output_path.to_str().unwrap(),
        ])
        .status()
        .context("Failed to optimize WASM")?;

    std::fs::rename(output_path, wasm_path).context("Failed to replace original WASM with optimized version")?;
    Ok(())
}

pub fn wasm_to_hex(wasm_path: &Path) -> Result<String> {
    let wasm_bytes = std::fs::read(wasm_path).context("Failed to read WASM file")?;
    Ok(hex::encode(&wasm_bytes))
}

pub fn copy_to_clipboard(text: &str) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        use std::io::Write;
        let mut child = Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .context("Failed to spawn pbcopy")?;
        
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())?;
        }
        
        child.wait().context("Failed to run pbcopy")?;
    }

    Ok(())
} 