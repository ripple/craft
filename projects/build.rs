use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Only build smart escrows if we're building the workspace root
    // (not when building individual workspace members)
    if env::var("CARGO_PKG_NAME").unwrap_or_default() != "craft-projects" {
        return;
    }

    println!("cargo:rerun-if-changed=examples/smart-escrows");
    
    let smart_escrows_dir = Path::new("examples/smart-escrows");
    
    if !smart_escrows_dir.exists() {
        println!("cargo:warning=Smart escrows directory not found, skipping");
        return;
    }

    let projects = [
        "kyc",
        "ledger_sqn", 
        "nft_owner",
        "notary",
        "notary_macro_example",
        "oracle",
    ];

    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let target = env::var("TARGET").unwrap_or_else(|_| "native".to_string());

    println!("cargo:warning=Building smart escrow examples (profile: {}, target: {})", profile, target);

    for project in &projects {
        let project_dir = smart_escrows_dir.join(project);
        
        if !project_dir.exists() {
            println!("cargo:warning=Project directory {} does not exist, skipping", project_dir.display());
            continue;
        }

        println!("cargo:warning=Building smart escrow: {}", project);

        // Build the project
        let mut cmd = Command::new("cargo");
        cmd.arg("build")
           .current_dir(&project_dir);

        // Add profile flag if release
        if profile == "release" {
            cmd.arg("--release");
        }

        // Add target flag if building for WASM
        if target.contains("wasm32") {
            cmd.arg("--target").arg(&target);
        }

        let output = cmd.output();
        
        match output {
            Ok(output) => {
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    println!("cargo:warning=Failed to build {}: {}\n{}", project, stderr, stdout);
                    // Don't panic, just warn and continue
                } else {
                    println!("cargo:warning=Successfully built {}", project);
                }
            }
            Err(e) => {
                println!("cargo:warning=Failed to execute cargo for {}: {}", project, e);
            }
        }
    }

    println!("cargo:warning=Finished building smart escrow examples");
}
