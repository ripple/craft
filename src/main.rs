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
    Build,
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
    /// Set up and run the XRPL Explorer
    StartExplorer {
        /// Run in background mode without visible console output
        #[arg(short, long)]
        background: bool,
    },
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
            Commands::Build => {
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
                        commands::test(&wasm_path, None, None, None).await?;
                    }
                    _ => (),
                }
            }
            Commands::Configure => {
                commands::configure().await?;
                println!("{}", "Configuration saved!".green());
            }
            Commands::Test { function, project, test_case, host_function_test } => {
                let config = if let Some(project_name) = project {
                    commands::configure_non_interactive(&project_name).await?
                } else {
                    commands::configure().await?
                };
                let wasm_path = commands::build(&config).await?;
                commands::test(&wasm_path, function, test_case, host_function_test).await?;
            }
            Commands::StartRippled { foreground } => {
                commands::start_rippled_with_foreground(foreground).await?;
            }
            Commands::ListRippled => {
                commands::list_rippled().await?;
            }
            Commands::StartExplorer { background } => {
                commands::start_explorer(background).await?;
            }
        },
        None => {
            // Nothing from the CLI was provided, so we'll interactively ask the user what they want to do
            let choices = vec![
                "Build WASM module",
                "Test WASM library function",
                "Start rippled",
                "List rippled processes",
                "Start Explorer",
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
                            commands::test(&wasm_path, None, None, None).await?;
                        }
                        _ => (),
                    }
                }
                "Test WASM library function" => {
                    let config = commands::configure().await?;
                    let wasm_path = commands::build(&config).await?;
                    commands::test(&wasm_path, None, None, None).await?;
                }
                "Start rippled" => {
                    let foreground = Confirm::new("Run rippled in foreground with console output? (Can be terminated with Ctrl+C)")
                        .with_default(true)
                        .prompt()?;

                    commands::start_rippled_with_foreground(foreground).await?;
                }
                "List rippled processes" => {
                    commands::list_rippled().await?;
                }
                "Start Explorer" => {
                    let background = Confirm::new(
                        "Run Explorer in background mode without visible console output",
                    )
                    .with_default(false)
                    .prompt()?;

                    commands::start_explorer(background).await?;
                }
                _ => (),
            }
        }
    }

    Ok(())
}
