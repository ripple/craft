mod commands;
mod config;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use inquire::Confirm;
use inquire::Select;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Build a WASM module
    Build {
        /// Project to build (non-interactive)
        #[arg(short, long)]
        project: Option<String>,
        /// Build in release mode
        #[arg(short, long)]
        release: bool,
        /// Optimization level (z, s, 0, 1, 2, 3)
        #[arg(long)]
        opt_level: Option<String>,
    },
    /// Configure build settings
    Configure,
    /// Test a WASM smart contract
    Test {
        /// Function to test
        #[arg(short, long)]
        function: Option<String>,
        /// Project to test (non-interactive)
        #[arg(short, long)]
        project: Option<String>,
        /// Test case to run (non-interactive)
        #[arg(short, long)]
        test_case: Option<String>,
        /// Host function test to run (uses new verification system)
        #[arg(long)]
        host_function_test: Option<String>,
    },
    /// Check if rippled is running and start it if not
    StartRippled {
        /// Run rippled in foreground with visible console output (can be terminated with Ctrl+C)
        #[arg(short, long)]
        foreground: bool,
    },
    /// List running rippled processes and show how to terminate them
    ListRippled,
    /// Open the XRPL Explorer for a local rippled instance
    OpenExplorer,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Check if the CLI binary needs to be updated
    if utils::needs_cli_update().unwrap_or(false) {
        println!(
            "{}",
            "\nDetected changes to the CLI source code that haven't been installed yet.".yellow()
        );
        if Confirm::new("Would you like to update the CLI now with 'cargo install --path .'?")
            .with_default(true)
            .prompt()?
        {
            utils::install_cli()?;
            println!(
                "{}",
                "Please run the command again to use the updated version.".blue()
            );
            return Ok(());
        }
    }

    let cli = Cli::parse();

    match cli.command {
        Some(cmd) => match cmd {
            Commands::Build {
                project,
                release,
                opt_level,
            } => {
                let config = if let Some(ref project_name) = project {
                    commands::configure_non_interactive_build(project_name, release, opt_level)
                        .await?
                } else {
                    commands::configure().await?
                };
                let wasm_path = commands::build(&config).await?;

                if !matches!(config.optimization_level, config::OptimizationLevel::None) {
                    commands::optimize(&wasm_path, &config.optimization_level).await?;
                }

                // If project was specified (non-interactive), just exit after build
                if project.is_some() {
                    println!("{}", "Build completed!".green());
                    return Ok(());
                }

                // After build, ask what to do next
                let choices = vec![
                    "Deploy to WASM Devnet",
                    "Test WASM library function",
                    "Exit",
                ];

                match Select::new("What would you like to do next?", choices).prompt()? {
                    "Deploy to WASM Devnet" => {
                        commands::deploy_to_wasm_devnet(&wasm_path).await?;
                    }
                    "Test WASM library function" => {
                        let (final_test_case, final_host_function_test) =
                            commands::test(&wasm_path, None, None, None).await?;

                        // Show equivalent command line for test
                        let project_name = utils::get_project_name(&config.project_path)
                            .unwrap_or("unknown".to_string());
                        let mut cmd = format!("craft test --project {}", project_name);

                        if let Some(tc) = final_test_case {
                            cmd.push_str(&format!(" --test-case {}", tc));
                        }
                        if let Some(hft) = final_host_function_test {
                            cmd.push_str(&format!(" --host-function-test {}", hft));
                        }

                        println!("\n{}", "💡 Equivalent command line:".blue());
                        println!("{}", cmd.green());
                    }
                    _ => (),
                }
            }
            Commands::Configure => {
                commands::configure().await?;
                println!("{}", "Configuration saved!".green());
            }
            Commands::Test {
                function,
                project,
                test_case,
                host_function_test,
            } => {
                let config = if let Some(ref project_name) = project {
                    commands::configure_non_interactive(project_name).await?
                } else {
                    commands::configure().await?
                };
                let wasm_path = commands::build(&config).await?;
                let (final_test_case, final_host_function_test) = commands::test(
                    &wasm_path,
                    function.clone(),
                    test_case.clone(),
                    host_function_test.clone(),
                )
                .await?;

                // Show equivalent command line if in interactive mode
                if project.is_none() {
                    let project_name = utils::get_project_name(&config.project_path)
                        .unwrap_or("unknown".to_string());
                    let mut cmd = format!("craft test --project {}", project_name);

                    if let Some(func) = function {
                        cmd.push_str(&format!(" --function {}", func));
                    }
                    if let Some(tc) = final_test_case {
                        cmd.push_str(&format!(" --test-case {}", tc));
                    }
                    if let Some(hft) = final_host_function_test {
                        cmd.push_str(&format!(" --host-function-test {}", hft));
                    }

                    println!("\n{}", "💡 Equivalent command line:".blue());
                    println!("{}", cmd.green());
                }
            }
            Commands::StartRippled { foreground } => {
                commands::start_rippled_with_foreground(foreground).await?;
            }
            Commands::ListRippled => {
                commands::list_rippled().await?;
            }
            Commands::OpenExplorer => {
                commands::open_explorer().await?;
            }
        },
        None => {
            // Nothing from the CLI was provided, so we'll interactively ask the user what they want to do
            let choices = vec![
                "Build WASM module",
                "Test WASM library function",
                "Start rippled",
                "List rippled processes",
                "Open Explorer",
                "Exit",
            ];

            match Select::new("What would you like to do?", choices).prompt()? {
                "Build WASM module" => {
                    let config = commands::configure().await?;
                    let wasm_path = commands::build(&config).await?;

                    if !matches!(config.optimization_level, config::OptimizationLevel::None) {
                        commands::optimize(&wasm_path, &config.optimization_level).await?;
                    }

                    // After build, ask what to do next
                    let choices = vec![
                        "Deploy to WASM Devnet",
                        "Test WASM library function",
                        "Exit",
                    ];

                    match Select::new("What would you like to do next?", choices).prompt()? {
                        "Deploy to WASM Devnet" => {
                            commands::deploy_to_wasm_devnet(&wasm_path).await?;
                        }
                        "Test WASM library function" => {
                            let _ = commands::test(&wasm_path, None, None, None).await?;
                        }
                        _ => (),
                    }
                }
                "Test WASM library function" => {
                    let config = commands::configure().await?;
                    let wasm_path = commands::build(&config).await?;
                    let (final_test_case, final_host_function_test) =
                        commands::test(&wasm_path, None, None, None).await?;

                    // Show equivalent command line
                    let project_name = utils::get_project_name(&config.project_path)
                        .unwrap_or("unknown".to_string());
                    let mut cmd = format!("craft test --project {}", project_name);

                    if let Some(tc) = final_test_case {
                        cmd.push_str(&format!(" --test-case {}", tc));
                    }
                    if let Some(hft) = final_host_function_test {
                        cmd.push_str(&format!(" --host-function-test {}", hft));
                    }

                    println!("\n{}", "💡 Equivalent command line:".blue());
                    println!("{}", cmd.green());
                }
                "Start rippled" => {
                    let foreground = Confirm::new("Run rippled in foreground with console output? (Can be terminated with Ctrl+C)")
                        .with_default(true)
                        .prompt()?;

                    commands::start_rippled_with_foreground(foreground).await?;

                    // Show equivalent command line
                    let cmd = if foreground {
                        "craft start-rippled --foreground".to_string()
                    } else {
                        "craft start-rippled".to_string()
                    };
                    println!("\n{}", "💡 Equivalent command line:".blue());
                    println!("{}", cmd.green());
                }
                "List rippled processes" => {
                    commands::list_rippled().await?;
                }
                "Open Explorer" => {
                    commands::open_explorer().await?;
                }
                _ => (),
            }
        }
    }

    Ok(())
}
