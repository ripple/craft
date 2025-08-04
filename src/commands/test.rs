use anyhow::Result;
use colored::*;
use std::path::Path;
use std::process::Command;

/// A unified test runner that simplifies wasm-host testing
pub struct TestRunner {
    wasm_path: std::path::PathBuf,
    project: String,
    verbose: bool,
}

impl TestRunner {
    pub fn new(wasm_path: &Path, project: &str) -> Self {
        Self {
            wasm_path: wasm_path.to_path_buf(),
            project: project.to_string(),
            verbose: false,
        }
    }

    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Run a single test case
    pub fn run_test(&self, test_case: &str, function: Option<&str>) -> Result<TestResult> {
        println!("{}", format!("Running test case: {test_case}").cyan());

        // Ensure wasm-host is built
        self.ensure_wasm_host_built()?;

        let wasm_host_path = self.get_wasm_host_path()?;

        let mut args = vec![
            "--wasm-file",
            self.wasm_path.to_str().unwrap(),
            "--test-case",
            test_case,
            "--project",
            &self.project,
        ];

        if let Some(func) = function {
            args.push("--function");
            args.push(func);
        }

        if self.verbose {
            args.push("--verbose");
        }

        let output = Command::new(&wasm_host_path)
            .args(&args)
            .output()
            .map_err(|e| anyhow::anyhow!("Failed to run wasm-host: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        Ok(TestResult {
            success: output.status.success(),
            test_case: test_case.to_string(),
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
            error_code: self.extract_error_code(&stderr),
        })
    }

    /// Run all test cases for a project
    pub fn run_all_tests(&self, project: &str) -> Result<Vec<TestResult>> {
        let test_cases = super::discover_test_cases(project)?;
        let mut results = Vec::new();

        println!(
            "{}",
            format!(
                "Running {} test cases for project '{}'",
                test_cases.len(),
                project
            )
            .cyan()
        );
        println!();

        for test_case in test_cases {
            let result = self.run_test(&test_case, None)?;
            let status_icon = if result.success { "✅" } else { "❌" };

            println!(
                "{} {} - {}",
                status_icon,
                result.test_case,
                if result.success {
                    "PASSED".green()
                } else {
                    "FAILED".red()
                }
            );

            if !result.success && self.verbose {
                if let Some(code) = &result.error_code {
                    println!("  Error code: {}", code.yellow());
                }
            }

            results.push(result);
        }

        // Summary
        let passed = results.iter().filter(|r| r.success).count();
        let failed = results.len() - passed;

        println!();
        println!("{}", "Test Summary:".bold());
        println!("  {} Passed: {}", "•".green(), passed);
        println!("  {} Failed: {}", "•".red(), failed);

        if failed > 0 && !self.verbose {
            println!();
            println!(
                "{}",
                "Run with --verbose to see detailed error output".yellow()
            );
        }

        Ok(results)
    }

    fn ensure_wasm_host_built(&self) -> Result<()> {
        let wasm_host_path = self.get_wasm_host_path()?;

        if !wasm_host_path.exists() {
            println!("{}", "Building wasm-host testing environment...".yellow());

            let status = Command::new("cargo")
                .args(["build", "--release", "-p", "wasm-host"])
                .status()
                .map_err(|e| anyhow::anyhow!("Failed to run cargo build: {}", e))?;

            if !status.success() {
                return Err(anyhow::anyhow!(
                    "Failed to build wasm-host. Make sure you're in the workspace root directory."
                ));
            }

            println!("{}", "wasm-host built successfully!".green());
        }

        Ok(())
    }

    fn get_wasm_host_path(&self) -> Result<std::path::PathBuf> {
        let path = std::env::current_dir()?
            .join("target")
            .join("release")
            .join("wasm-host");
        Ok(path)
    }

    fn extract_error_code(&self, stderr: &str) -> Option<String> {
        stderr
            .lines()
            .find(|line| line.contains("error code:"))
            .and_then(|line| line.split("error code:").nth(1))
            .map(|s| s.trim().to_string())
    }
}

/// Result of a test run
pub struct TestResult {
    pub success: bool,
    pub test_case: String,
    pub stdout: String,
    pub stderr: String,
    pub error_code: Option<String>,
}

#[allow(dead_code)]
impl TestResult {
    /// Get a human-readable description of the error
    pub fn error_description(&self) -> Option<String> {
        self.error_code.as_ref().map(|code| match code.as_str() {
            "-11" => "Field not found or wrong size".to_string(),
            "-101" => "Failed to get transaction field".to_string(),
            "-102" => "Transaction field has unexpected value".to_string(),
            "-201" => "Failed to get ledger object field".to_string(),
            "-202" => "Fee field test failed - XRP amounts should be exactly 8 bytes".to_string(),
            "-203" => "Failed to get NFT".to_string(),
            "-301" => "Failed to trace".to_string(),
            "-302" => "Failed to trace number".to_string(),
            "-401" => "Failed to update data".to_string(),
            "-501" => "Failed to get escrow condition".to_string(),
            "-601" => "Failed to get transaction amount".to_string(),
            "-701" => "Failed to get ledger objects".to_string(),
            _ if code.starts_with("-10") => "Transaction field access error".to_string(),
            _ if code.starts_with("-20") => "Ledger object access error".to_string(),
            _ if code.starts_with("-30") => "Trace/debug error".to_string(),
            _ if code.starts_with("-40") => "State update error".to_string(),
            _ if code.starts_with("-50") => "Condition access error".to_string(),
            _ if code.starts_with("-60") => "Amount access error".to_string(),
            _ if code.starts_with("-70") => "Ledger iteration error".to_string(),
            _ => format!("Unknown error code: {code}"),
        })
    }
}

/// Quick test helper for interactive use
#[allow(dead_code)]
pub async fn quick_test(project: &str, test_case: Option<&str>, verbose: bool) -> Result<()> {
    // Find the project's WASM file
    let project_path = std::env::current_dir()?.join("projects").join(project);
    let wasm_path = crate::utils::find_wasm_output(&project_path)?;

    let runner = TestRunner::new(&wasm_path, project).verbose(verbose);

    if let Some(case) = test_case {
        // Run single test
        let result = runner.run_test(case, None)?;

        // Print output
        println!("{}", result.stdout);

        if !result.success {
            println!("{}", result.stderr.red());

            if let Some(desc) = result.error_description() {
                println!();
                println!("{}: {}", "Error".red().bold(), desc);
            }

            return Err(anyhow::anyhow!("Test failed"));
        }
    } else {
        // Run all tests
        let results = runner.run_all_tests(project)?;

        // Check if any failed
        if results.iter().any(|r| !r.success) {
            return Err(anyhow::anyhow!("Some tests failed"));
        }
    }

    Ok(())
}
