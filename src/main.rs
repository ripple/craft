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
    /// Automatically accept CLI update prompts
    #[arg(short = 'y', long = "yes", global = true)]
    yes: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Build a WASM module
    Build {
        /// Project name under projects directory
        #[arg(index = 1)]
        project: Option<String>,
        /// Build mode (debug or release)
        #[arg(short='m', long, value_enum, default_value_t = config::BuildMode::Release)]
        mode: config::BuildMode,
        /// Optimization level (none, small, aggressive)
        #[arg(short='O', long, value_enum, default_value_t = config::OptimizationLevel::Small)]
        opt: config::OptimizationLevel,
    },
    /// Configure build settings
    Configure,
    /// Test a WASM smart contract
    Test {
        /// Function to test
        #[arg(short, long)]
        function: Option<String>,
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
    let cli = Cli::parse();

    // Check if the CLI binary needs to be updated
    if utils::needs_cli_update().unwrap_or(false) {
        // Check if we're running in a non-interactive environment
        let is_interactive = atty::is(atty::Stream::Stdin);

        if is_interactive || cli.yes {
            println!(
                "{}",
                "\nDetected changes to the CLI source code that haven't been installed yet."
                    .yellow()
            );

            // Auto-accept if -y flag is provided, otherwise prompt
            let should_update = if cli.yes {
                println!("Auto-accepting update due to -y flag...");
                true
            } else {
                Confirm::new("Would you like to update the CLI now with 'cargo install --path .'?")
                    .with_default(true)
                    .prompt()?
            };

            if should_update {
                utils::install_cli()?;
                println!(
                    "{}",
                    "Please run the command again to use the updated version.".blue()
                );
                return Ok(());
            }
        } else {
            // In non-interactive mode, just warn but continue
            eprintln!(
                "Warning: CLI source code has changed but running in non-interactive mode. \
                 Please run 'cargo install --path .' to update."
            );
        }
    }

    match cli.command {
        Some(cmd) => match cmd {
            Commands::Build { project, mode, opt } => {
                // Non-interactive build using CLI flags
                let project_path = if let Some(proj) = project {
                    std::env::current_dir()?.join("projects").join(proj)
                } else {
                    // Fallback to interactive selection
                    let config = commands::configure().await?;
                    commands::build(&config).await?;
                    return Ok(());
                };
                // Prepare configuration
                let config = config::Config {
                    project_path,
                    build_mode: mode,
                    optimization_level: opt,
                    ..Default::default()
                };
                // Execute build
                let wasm_path = commands::build(&config).await?;

                if !matches!(config.optimization_level, config::OptimizationLevel::None) {
                    commands::optimize(&wasm_path, &config.optimization_level).await?;
                }

                // Only show interactive menu if we're in an interactive terminal
                if atty::is(atty::Stream::Stdin) {
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
                            commands::test(&wasm_path, None).await?;
                        }
                        _ => (),
                    }
                }
            }
            Commands::Configure => {
                commands::configure().await?;
                println!("{}", "Configuration saved!".green());
            }
            Commands::Test { function } => {
                let config = commands::configure().await?;
                let wasm_path = commands::build(&config).await?;
                commands::test(&wasm_path, function).await?;
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
                            commands::test(&wasm_path, None).await?;
                        }
                        _ => (),
                    }
                }
                "Test WASM library function" => {
                    let config = commands::configure().await?;
                    let wasm_path = commands::build(&config).await?;
                    commands::test(&wasm_path, None).await?;
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
                "Open Explorer" => {
                    commands::open_explorer().await?;
                }
                _ => (),
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod cli_tests {
    use super::*;
    use crate::config::{BuildMode, OptimizationLevel};
    use clap::Parser;

    #[test]
    fn test_build_command_parsing() {
        let cli = Cli::parse_from([
            "craft", "build", "myproj", "--mode", "debug", "--opt", "none",
        ]);
        match cli.command {
            Some(Commands::Build { project, mode, opt }) => {
                assert_eq!(project.unwrap(), "myproj");
                assert_eq!(mode, BuildMode::Debug);
                assert_eq!(opt, OptimizationLevel::None);
            }
            other => panic!("Expected Build command, got: {:?}", other),
        }
    }

    #[test]
    fn test_build_defaults() {
        let cli = Cli::parse_from(["craft", "build"]);
        match cli.command {
            Some(Commands::Build { project, mode, opt }) => {
                assert!(project.is_none());
                assert_eq!(mode, BuildMode::Release);
                assert_eq!(opt, OptimizationLevel::Small);
            }
            other => panic!("Expected Build command, got: {:?}", other),
        }
    }

    #[test]
    fn test_build_with_positional_project() {
        let cli = Cli::parse_from(["craft", "build", "myproj", "--mode", "debug"]);
        match cli.command {
            Some(Commands::Build { project, mode, opt }) => {
                assert_eq!(project.unwrap(), "myproj");
                assert_eq!(mode, BuildMode::Debug);
                assert_eq!(opt, OptimizationLevel::Small);
            }
            other => panic!("Expected Build command, got: {:?}", other),
        }
    }
}
