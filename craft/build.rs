use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn collect_craft_package_paths(manifest_dir: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    // Add craft/Cargo.toml
    let craft_manifest = manifest_dir.join("Cargo.toml");
    if craft_manifest.exists() {
        paths.push(craft_manifest);
    }

    // Walk craft/src
    let src_dir = manifest_dir.join("src");
    if src_dir.exists() {
        let mut stack = vec![src_dir];
        while let Some(dir) = stack.pop() {
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        stack.push(path);
                    } else if path.is_file() {
                        paths.push(path);
                    }
                }
            }
        }
    }

    paths.sort();
    paths
}

fn compute_sources_fingerprint(paths: &[PathBuf], base: &Path) -> String {
    let mut hasher = Sha256::new();
    for p in paths {
        if let Ok(rel) = p.strip_prefix(base) {
            hasher.update(rel.to_string_lossy().as_bytes());
            hasher.update([0]);
        }
        if let Ok(bytes) = fs::read(p) {
            hasher.update(&bytes);
        }
        hasher.update([0xFF]);
    }
    let digest = hasher.finalize();
    hex::encode(digest)
}

fn main() {
    // Try to capture the current git commit hash
    if let Ok(out) = Command::new("git").args(["rev-parse", "HEAD"]).output()
        && out.status.success()
    {
        let hash = String::from_utf8_lossy(&out.stdout).trim().to_string();
        println!("cargo:rustc-env=BUILD_GIT_HASH={}", hash);
    }

    // Detect if the working tree is dirty at build time
    let dirty = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .ok()
        .map(|o| o.status.success() && !o.stdout.is_empty())
        .unwrap_or(false);
    println!(
        "cargo:rustc-env=BUILD_GIT_DIRTY={}",
        if dirty { "1" } else { "0" }
    );

    // Record build timestamp (unix seconds)
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    println!("cargo:rustc-env=BUILD_UNIX_TIME={}", now);

    // Compute deterministic craft source fingerprint at build time (craft package only)
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let base = PathBuf::from(&manifest_dir);
        let paths = collect_craft_package_paths(&base);
        let fp = compute_sources_fingerprint(&paths, &base);
        println!("cargo:rustc-env=BUILD_SOURCE_FINGERPRINT={}", fp);

        // Re-run build script when relevant craft package files change
        println!("cargo:rerun-if-changed={}", base.join("src").display());
        println!(
            "cargo:rerun-if-changed={}",
            base.join("Cargo.toml").display()
        );

        // Also re-run when HEAD changes (absolute path to workspace .git/HEAD if available)
        if let Some(ws_root) = Path::new(&base).parent() {
            let head = ws_root.join(".git/HEAD");
            if head.exists() {
                println!("cargo:rerun-if-changed={}", head.display());
            }
        }
    }

    // Manual override to force rebuild
    println!("cargo:rerun-if-env-changed=FORCE_REBUILD");
}
