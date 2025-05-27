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
        /// Project name under projects directory (can be positional or via -p/--project)
        #[arg(short, long, index = 1)]
        project: Option<String>,
        /// Build mode (debug or release)
        #[arg(short='m', long, value_enum, default_value_t = config::BuildMode::Release)]
        mode: config::BuildMode,
        /// Optimization level (none, small, aggressive)
        #[arg(short='O', long, value_enum, default_value_t = config::OptimizationLevel::Small)]
        opt: config::OptimizationLevel,
        /// Enable wee_alloc for smaller binary size
        #[arg(long, default_value_t = false)]
        wee_alloc: bool,
    },
    /// Configure build settings
    Configure,
    /// Export WASM as hex
    ExportHex,
    /// Setup wee_alloc for smaller binary size
    SetupWeeAlloc,
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
            Commands::Build {
                project,
                mode,
                opt,
                wee_alloc,
            } => {
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
                let mut config = config::Config::default();
                config.project_path = project_path;
                config.build_mode = mode;
                config.optimization_level = opt;
                config.use_wee_alloc = wee_alloc;
                // Execute build
                let wasm_path = commands::build(&config).await?;
                if config.use_wee_alloc {
                    commands::setup_wee_alloc(&config.project_path).await?;
                }
                return Ok(());
            }
            Commands::Configure => {
                commands::configure().await?;
                println!("{}", "Configuration saved!".green());
            }
            Commands::ExportHex => {
                let config = commands::configure().await?;
                let wasm_path = commands::build(&config).await?;
                commands::copy_wasm_hex_to_clipboard(&wasm_path).await?;
            }
            Commands::SetupWeeAlloc => {
                commands::setup_wee_alloc(&std::env::current_dir()?).await?;
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
            Commands::StartExplorer { background } => {
                commands::start_explorer(background).await?;
            }
        },
        None => {
            // Nothing from the CLI was provided, so we'll interactively ask the user what they want to do
            let choices = vec![
                "Build WASM module",
                "Test WASM library function",
                "Setup wee_alloc",
                "Start rippled",
                "List rippled processes",
                "Start Explorer",
                "Exit",
            ];

            match Select::new("What would you like to do?", choices).prompt()? {
                "Build WASM module" => {
                    let config = commands::configure().await?;
                    let wasm_path = commands::build(&config).await?;

                    if config.use_wee_alloc {
                        commands::setup_wee_alloc(&config.project_path).await?;
                    }

                    // After build, ask what to do next
                    let choices = vec![
                        "Deploy to WASM Devnet",
                        "Copy WASM hex to clipboard",
                        "Test WASM library function",
                        "Exit",
                    ];

                    match Select::new("What would you like to do next?", choices).prompt()? {
                        "Deploy to WASM Devnet" => {
                            commands::deploy_to_wasm_devnet(&wasm_path).await?;
                        }
                        "Copy WASM hex to clipboard" => {
                            commands::copy_wasm_hex_to_clipboard(&wasm_path).await?;
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
                "Setup wee_alloc" => {
                    commands::setup_wee_alloc(&std::env::current_dir()?).await?;
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

#[cfg(test)]
mod cli_tests {
    use super::*;
    use clap::Parser;
    use crate::config::{BuildMode, OptimizationLevel};

    #[test]
    fn test_build_command_parsing() {
        let cli = Cli::parse_from(&[
            "craft",
            "build",
            "--project",
            "myproj",
            "--mode",
            "debug",
            "--opt",
            "none",
            "--wee-alloc",
        ]);
        match cli.command {
            Some(Commands::Build { project, mode, opt, wee_alloc }) => {
                assert_eq!(project.unwrap(), "myproj");
                assert_eq!(mode, BuildMode::Debug);
                assert_eq!(opt, OptimizationLevel::None);
                assert!(wee_alloc);
            }
            other => panic!("Expected Build command, got: {:?}", other),
        }
    }

    #[test]
    fn test_build_defaults() {
        let cli = Cli::parse_from(&["craft", "build"]);
        match cli.command {
            Some(Commands::Build { project, mode, opt, wee_alloc }) => {
                assert!(project.is_none());
                assert_eq!(mode, BuildMode::Release);
                assert_eq!(opt, OptimizationLevel::Small);
                assert!(!wee_alloc);
            }
            other => panic!("Expected Build command, got: {:?}", other),
        }
    }

    #[test]
    fn test_build_with_positional_project() {
        let cli = Cli::parse_from(&[
            "craft", 
            "build",
            "myproj",
            "--mode",
            "debug"
        ]);
        match cli.command {
            Some(Commands::Build { project, mode, opt, wee_alloc }) => {
                assert_eq!(project.unwrap(), "myproj");
                assert_eq!(mode, BuildMode::Debug);
                assert_eq!(opt, OptimizationLevel::Small);
                assert!(!wee_alloc);
            }
            other => panic!("Expected Build command, got: {:?}", other),
        }
    }
}
