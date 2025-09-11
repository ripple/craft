use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;
use walkdir::WalkDir;
use which::which;

pub fn find_wasm_projects(base_path: &Path) -> Vec<PathBuf> {
    let mut projects = Vec::new();

    // First check projects/examples for all example projects
    let examples_path = base_path.join("projects/examples");
    if examples_path.exists() {
        // Walk through all subdirectories in examples
        if let Ok(entries) = std::fs::read_dir(&examples_path) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_dir() {
                    // Check direct children (like examples/foo)
                    if path.join("Cargo.toml").exists()
                        && is_valid_wasm_project(&path.join("Cargo.toml"))
                    {
                        projects.push(path.clone());
                    }

                    // Also check subdirectories (like examples/smart-escrows/notary)
                    if let Ok(sub_entries) = std::fs::read_dir(&path) {
                        for sub_entry in sub_entries.filter_map(|e| e.ok()) {
                            let sub_path = sub_entry.path();
                            if sub_path.is_dir()
                                && sub_path.join("Cargo.toml").exists()
                                && is_valid_wasm_project(&sub_path.join("Cargo.toml"))
                            {
                                projects.push(sub_path);
                            }
                        }
                    }
                }
            }
        }
    }

    // Also check projects/ root for any direct WASM projects
    let projects_path = base_path.join("projects");
    if projects_path.exists()
        && let Ok(entries) = std::fs::read_dir(&projects_path)
    {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir()
                && !path.ends_with("examples")
                && path.join("Cargo.toml").exists()
                && is_valid_wasm_project(&path.join("Cargo.toml"))
            {
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

pub fn is_valid_rust_project(path: &Path) -> bool {
    // Check if there's a Cargo.toml in this directory
    let cargo_toml_path = path.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        // Also check parent directory
        if let Some(parent) = path.parent() {
            let parent_cargo = parent.join("Cargo.toml");
            if parent_cargo.exists() {
                return is_valid_wasm_project(&parent_cargo);
            }
        }
        return false;
    }

    is_valid_wasm_project(&cargo_toml_path)
}

fn is_valid_wasm_project(cargo_toml_path: &Path) -> bool {
    // Read Cargo.toml and check if it's configured for WASM
    if let Ok(content) = std::fs::read_to_string(cargo_toml_path) {
        // Check for cdylib crate type which is required for WASM
        content.contains("cdylib") ||
        // Also accept projects that might have wasm-related dependencies
        content.contains("wasm-bindgen") ||
        content.contains("wasm32-unknown-unknown")
    } else {
        false
    }
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
        .context(format!("Failed to install WASM target: {target}"))?;
    Ok(())
}

pub fn check_wasm_opt_installed() -> bool {
    which("wasm-opt").is_ok()
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
        anyhow::bail!(
            "No Cargo.toml found in {}. This is not a valid Rust project directory.",
            project_path.display()
        );
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
                &format!("name = \"{package_name}\""),
                &format!("name = \"{fixed_name}\""),
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
                    format!("\nError: A folder named '{updated_package_name}' already exists.")
                        .red()
                );
                println!("{}", "\nSuggestions:".yellow());
                println!("  • Choose a different package name in Cargo.toml");
                println!("  • Remove or rename the existing directory");
                println!("  • Continue with the current folder name");
                return Ok(project_path.to_path_buf());
            }

            // Rename the directory
            std::fs::rename(project_path, &new_path)?;
            println!(
                "{}",
                format!(
                    "\nRenamed folder from '{project_folder_name}' to '{updated_package_name}'!"
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

/// Returns a user-facing message describing why an update is recommended, or None if up-to-date
pub fn cli_update_status() -> Result<Option<String>> {
    use std::time::{SystemTime, UNIX_EPOCH};

    // Allow disabling update checks (useful for CI/non-dev usage)
    if std::env::var_os("CRAFT_DISABLE_UPDATE_CHECK").is_some() {
        return Ok(None);
    }

    let workspace_dir = env::current_dir().context("Failed to get current directory")?;
    let in_project_root =
        workspace_dir.join("craft").exists() && workspace_dir.join("Cargo.toml").exists();

    // Helper to get current HEAD hash
    let git_head = || -> Option<String> {
        let out = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .ok()?;
        if out.status.success() {
            Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
        } else {
            None
        }
    }();

    // Helper to detect dirty working tree
    let git_dirty = || -> Option<bool> {
        let out = Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .ok()?;
        if out.status.success() {
            Some(!out.stdout.is_empty())
        } else {
            None
        }
    }();

    // Helper to get timestamp of latest commit
    let git_head_time = || -> Option<u64> {
        let out = Command::new("git")
            .args(["log", "-1", "--format=%ct"])
            .output()
            .ok()?;
        if !out.status.success() {
            return None;
        }
        let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
        s.parse::<u64>().ok()
    }();

    // Build-time metadata injected by build.rs (if present)
    let build_hash = option_env!("BUILD_GIT_HASH").map(|s| s.to_string());
    let build_dirty = option_env!("BUILD_GIT_DIRTY")
        .map(|s| s == "1")
        .unwrap_or(false);
    let build_unix_time = option_env!("BUILD_UNIX_TIME")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        });
    let build_source_fp = option_env!("BUILD_SOURCE_FINGERPRINT").map(|s| s.to_string());

    // If we can compute a deterministic fingerprint of the craft sources and it matches
    // what the binary was built from, we consider it up-to-date even if the workspace is dirty.
    if in_project_root {
        let current_fp = {
            // Collect relevant craft package sources (must match build.rs logic)
            let craft_dir = workspace_dir.join("craft");
            let mut paths: Vec<PathBuf> = vec![craft_dir.join("Cargo.toml")];
            let src_dir = craft_dir.join("src");
            if src_dir.exists() {
                for entry in WalkDir::new(&src_dir).into_iter().filter_map(|e| e.ok()) {
                    if entry.path().is_file() {
                        paths.push(entry.path().to_path_buf());
                    }
                }
            }
            paths.sort();
            // Compute SHA-256 over relative path + NUL + bytes per file
            let mut hasher = Sha256::new();
            for p in &paths {
                if let Ok(rel) = p.strip_prefix(&craft_dir) {
                    hasher.update(rel.to_string_lossy().as_bytes());
                    hasher.update([0]);
                }
                if let Ok(bytes) = fs::read(p) {
                    hasher.update(&bytes);
                }
                hasher.update([0xFF]);
            }
            hex::encode(hasher.finalize())
        };

        if let Some(build_fp) = build_source_fp.clone()
            && build_fp == current_fp
        {
            return Ok(None);
        }
    }

    // Prefer a robust git-based comparison when in repo root and git is usable
    if in_project_root && let Some(head) = git_head {
        let dirty = git_dirty.unwrap_or(false);
        let head_time = git_head_time.unwrap_or(build_unix_time);

        // If working tree is dirty, recommend update (dev workflow)
        if dirty {
            return Ok(Some(format!(
                "Detected uncommitted changes in the repository. The craft binary may be stale.\n  Built from: {}{}\n  Workspace:  {} (dirty)\n  Tip: Reinstall with: cargo install --path craft",
                build_hash.clone().unwrap_or_else(|| "unknown".to_string()),
                if build_dirty { " (dirty)" } else { "" },
                head
            )));
        }

        // If built from different commit, recommend update
        if build_hash.as_deref() != Some(head.as_str()) {
            return Ok(Some(format!(
                "Newer source detected than the installed craft binary.\n  Built from: {}{}\n  Workspace:  {}\n  Tip: Reinstall with: cargo install --path craft",
                build_hash.unwrap_or_else(|| "unknown".to_string()),
                if build_dirty { " (dirty)" } else { "" },
                head
            )));
        }

        // If latest commit time is after build time, recommend update
        if head_time > build_unix_time {
            return Ok(Some(
                "Source repository has newer commits than this binary's build. Tip: cargo install --path craft".to_string(),
            ));
        }

        // Up to date according to git metadata
        return Ok(None);
    }

    // Fallback: compare source file mtimes to the current binary mtime
    let current_exe = env::current_exe().context("Failed to get current executable path")?;
    let binary_modified = current_exe
        .metadata()
        .context("Failed to get binary metadata")?
        .modified()
        .context("Failed to get binary modification time")?;

    let source_files = vec![
        workspace_dir.join("craft/src"),
        workspace_dir.join("craft/Cargo.toml"),
        workspace_dir.join("Cargo.toml"),
    ];

    // Walk directories and check mtimes
    for path in source_files {
        if !path.exists() {
            continue;
        }
        if path.is_file() {
            let src_m = path.metadata()?.modified()?;
            if src_m > binary_modified {
                return Ok(Some(
                    "Source files are newer than the craft binary. Tip: cargo install --path craft"
                        .into(),
                ));
            }
        } else {
            for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                if entry.path().is_file() {
                    let src_m = entry.metadata()?.modified()?;
                    if src_m > binary_modified {
                        return Ok(Some("Source files are newer than the craft binary. Tip: cargo install --path craft".into()));
                    }
                }
            }
        }
    }

    Ok(None)
}

/// Installs the CLI from the current directory
pub fn install_cli() -> Result<()> {
    use colored::*;
    use std::process::Stdio;

    println!("{}", "Installing updated craft CLI...".blue());

    // Run cargo install with real-time output
    let status = Command::new("cargo")
        .args(["install", "--path", "craft"])
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

/// Run cargo fmt in the current directory
pub fn run_cargo_fmt() -> Result<()> {
    use colored::*;

    println!("{}", "Running cargo fmt...".cyan());

    let status = Command::new("cargo")
        .args(["fmt"])
        .status()
        .context("Failed to run cargo fmt")?;

    if status.success() {
        println!("{}", "✅ Code formatted successfully!".green());
    } else {
        println!("{}", "⚠️  cargo fmt encountered issues".yellow());
    }

    Ok(())
}

/// Find WASM output file for a project
pub fn find_wasm_output(project_path: &Path) -> Result<PathBuf> {
    let cargo_toml = find_cargo_toml(project_path).context("Could not find Cargo.toml")?;
    let project_dir = cargo_toml.parent().unwrap();
    let project_name = project_dir.file_name().unwrap().to_str().unwrap();
    let project_main_dir = project_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    // Try release first, then debug
    let candidates = vec![
        project_dir
            .join("target/wasm32-unknown-unknown/release")
            .join(format!("{project_name}.wasm")),
        project_dir
            .join("target/wasm32-unknown-unknown/release")
            .join(format!("lib{project_name}.wasm")),
        project_dir
            .join("target/wasm32-unknown-unknown/debug")
            .join(format!("{project_name}.wasm")),
        project_dir
            .join("target/wasm32-unknown-unknown/debug")
            .join(format!("lib{project_name}.wasm")),
        project_main_dir
            .join("target/wasm32-unknown-unknown/release")
            .join(format!("{project_name}.wasm")),
        project_main_dir
            .join("target/wasm32-unknown-unknown/release")
            .join(format!("lib{project_name}.wasm")),
        project_main_dir
            .join("target/wasm32-unknown-unknown/debug")
            .join(format!("{project_name}.wasm")),
        project_main_dir
            .join("target/wasm32-unknown-unknown/debug")
            .join(format!("lib{project_name}.wasm")),
    ];

    for candidate in candidates {
        if candidate.exists() {
            return Ok(candidate);
        }
    }

    Err(anyhow::anyhow!(
        "No WASM output found for project '{}'.\n\n{}\n  • Run: craft build {}\n  • Make sure the project has a [lib] section in Cargo.toml\n  • Check if the build target is set to wasm32-unknown-unknown\n  • Look for .wasm files in target/wasm32-unknown-unknown/",
        project_name,
        "Suggestions:",
        project_name
    ))
}

pub mod wasm_fingerprint;
pub use wasm_fingerprint::calculate_wasm_fingerprint;
