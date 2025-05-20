use anyhow::{Context, Result};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;
use walkdir::WalkDir;

pub fn find_wasm_projects(base_path: &Path) -> Vec<PathBuf> {
    let mut projects = Vec::new();

    if let Ok(entries) = std::fs::read_dir(base_path.join("projects")) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() && path.join("Cargo.toml").exists() {
                projects.push(path);
            }
        }
    }

    projects
}

pub fn get_project_name(path: &Path) -> Option<String> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|s| s.to_string())
}

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

    std::fs::rename(output_path, wasm_path)
        .context("Failed to replace original WASM with optimized version")?;
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
    #[cfg(target_os = "linux")]
    {
        use std::io::Write;

        let mut child = Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .context("Failed to spawn xclip")?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(text.as_bytes())
                .context("Failed to write to xclip stdin")?;
        }

        child.wait().context("Failed to wait on xclip")?;
    }

    Ok(())
}

pub fn validate_project_name(project_path: &Path) -> Result<PathBuf> {
    let project_folder_name = get_project_name(project_path).unwrap_or_default();
    let cargo_toml_path = project_path.join("Cargo.toml");

    if !cargo_toml_path.exists() {
        return Ok(project_path.to_path_buf());
    }

    let cargo_content = std::fs::read_to_string(&cargo_toml_path)?;
    let name_pattern = regex::Regex::new(r#"name\s*=\s*"([^"]*)""#)?;

    let package_name = if let Some(caps) = name_pattern.captures(&cargo_content) {
        caps.get(1)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default()
    } else {
        project_folder_name.clone()
    };

    let mut updated_package_name = package_name.clone();
    let mut package_updated = false;

    // Check for hyphens in package name
    if package_name.contains('-') {
        use colored::*;
        use inquire::Confirm;

        println!("{}", "\nWarning: Package name contains hyphens.".yellow());
        println!("In Rust, crate names with hyphens can cause issues with WASM output filenames.");

        let fixed_name = package_name.replace('-', "_");

        println!("\nCurrent package name: {}", package_name.white().bold());
        println!("Suggested name:       {}", fixed_name.green().bold());

        if Confirm::new("Would you like to update the package name in Cargo.toml?")
            .with_default(true)
            .prompt()?
        {
            let updated_content = cargo_content.replace(
                &format!("name = \"{}\"", package_name),
                &format!("name = \"{}\"", fixed_name),
            );

            std::fs::write(&cargo_toml_path, updated_content)?;
            println!("{}", "\nUpdated package name in Cargo.toml!".green());

            updated_package_name = fixed_name;
            package_updated = true;
        }
    }

    // Check if folder name matches package name
    if project_folder_name != updated_package_name {
        use colored::*;
        use inquire::Confirm;

        println!(
            "{}",
            "\nWarning: Folder name doesn't match package name.".yellow()
        );
        println!("This can cause confusion and issues with WASM output filenames.");

        println!(
            "\nCurrent folder name: {}",
            project_folder_name.white().bold()
        );
        println!(
            "Package name:        {}",
            updated_package_name.green().bold()
        );

        if Confirm::new("Would you like to rename the folder to match the package name?")
            .with_default(true)
            .prompt()?
        {
            // Get the parent directory
            let parent_dir = project_path.parent().unwrap_or(Path::new(""));
            let new_path = parent_dir.join(&updated_package_name);

            // Check if destination already exists
            if new_path.exists() {
                println!(
                    "{}",
                    format!(
                        "\nError: A folder named '{}' already exists.",
                        updated_package_name
                    )
                    .red()
                );
                return Ok(project_path.to_path_buf());
            }

            // Rename the directory
            std::fs::rename(project_path, &new_path)?;
            println!(
                "{}",
                format!(
                    "\nRenamed folder from '{}' to '{}'!",
                    project_folder_name, updated_package_name
                )
                .green()
            );

            return Ok(new_path);
        }
    }

    // If we only updated the package name but not the folder, return the original path
    if package_updated {
        let parent = project_path.parent().unwrap_or(Path::new(""));
        return Ok(parent.join(updated_package_name));
    }

    Ok(project_path.to_path_buf())
}

/// Checks if the installed CLI binary is outdated compared to the source code
pub fn needs_cli_update() -> Result<bool> {
    use colored::*;

    // Get the path to the currently running binary
    let current_exe = env::current_exe().context("Failed to get current executable path")?;

    // Get the timestamp of the current binary
    let binary_modified = current_exe
        .metadata()
        .context("Failed to get binary metadata")?
        .modified()
        .context("Failed to get binary modification time")?;

    // Get paths to important source files
    let workspace_dir = env::current_dir().context("Failed to get current directory")?;

    // Check if we're in the project root directory
    let is_in_project_root =
        workspace_dir.join("src").exists() && workspace_dir.join("Cargo.toml").exists();

    if !is_in_project_root {
        println!(
            "{}",
            "\nWarning: Not running from the project root directory.".yellow()
        );
        println!(
            "Update detection may not work correctly. Current dir: {}",
            workspace_dir.display()
        );
    }

    let source_files = vec![
        workspace_dir.join("src/main.rs"),
        workspace_dir.join("src/commands/mod.rs"),
        workspace_dir.join("src/utils/mod.rs"),
        workspace_dir.join("src/config/mod.rs"),
        workspace_dir.join("Cargo.toml"),
    ];

    // Check if any source file is newer than the binary
    for source_file in source_files {
        if !source_file.exists() {
            continue;
        }

        let source_modified = source_file
            .metadata()
            .context(format!("Failed to get metadata for {:?}", source_file))?
            .modified()
            .context(format!(
                "Failed to get modification time for {:?}",
                source_file
            ))?;

        // If source file is newer than binary, update is needed
        if source_modified > binary_modified {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Installs the CLI from the current directory
pub fn install_cli() -> Result<()> {
    use colored::*;
    use std::process::Stdio;

    println!("{}", "Installing updated craft CLI...".blue());

    // Run cargo install with real-time output
    let status = Command::new("cargo")
        .args(["install", "--path", "."])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to run cargo install")?;

    if status.success() {
        // Add minimal delay to ensure filesystem updates timestamp
        thread::sleep(Duration::from_millis(10));
        println!("{}", "✅ craft CLI updated successfully!".green());
    } else {
        println!("{}", "❌ Failed to update craft CLI".red());
        return Err(anyhow::anyhow!("Installation failed"));
    }

    Ok(())
}

pub mod wasm_fingerprint;
pub use wasm_fingerprint::calculate_wasm_fingerprint;
