mod commands;
mod config;
mod docker;
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
    /// Export WASM as hex
    ExportHex,
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
    /// Stop the rippled Docker container
    StopRippled,
    /// Advance the ledger in stand-alone mode
    AdvanceLedger {
        /// Number of ledgers to advance (default: 1)
        #[arg(short, long, default_value = "1")]
        count: u32,
    },
    /// Manage Docker runtime (Colima)
    Docker {
        #[command(subcommand)]
        action: Option<DockerAction>,
    },
    /// Open the XRPL Explorer for a local rippled instance
    OpenExplorer,
}

#[derive(Subcommand)]
enum DockerAction {
    /// Install Colima (lightweight Docker runtime)
    Install,
    /// Start Colima
    Start,
    /// Stop Colima
    Stop,
    /// Check Docker status
    Status,
}

async fn handle_docker_command(action: Option<DockerAction>) -> Result<()> {
    use std::process::Command;

    match action {
        Some(DockerAction::Install) => {
            println!(
                "{}",
                "Installing Colima (lightweight Docker runtime)...".cyan()
            );

            // Check if Homebrew is installed
            let brew_check = Command::new("which")
                .arg("brew")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);

            if !brew_check {
                anyhow::bail!(
                    "Homebrew is not installed. Please install it first:\n\
                    /bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
                );
            }

            // Install Colima and Docker CLI
            let status = Command::new("brew")
                .args(["install", "colima", "docker"])
                .status()?;

            if status.success() {
                println!("{}", "Colima installed successfully!".green());
                println!("{}", "Run 'craft docker start' to start Colima.".blue());
            } else {
                anyhow::bail!("Failed to install Colima");
            }
        }
        Some(DockerAction::Start) => {
            println!("{}", "Starting Colima...".cyan());
            let status = Command::new("colima").arg("start").status()?;

            if status.success() {
                println!("{}", "Colima started successfully!".green());
            } else {
                anyhow::bail!("Failed to start Colima");
            }
        }
        Some(DockerAction::Stop) => {
            println!("{}", "Stopping Colima...".cyan());
            let status = Command::new("colima").arg("stop").status()?;

            if status.success() {
                println!("{}", "Colima stopped successfully!".green());
            } else {
                anyhow::bail!("Failed to stop Colima");
            }
        }
        Some(DockerAction::Status) | None => {
            // Check Docker status
            println!("{}", "Checking Docker status...".cyan());

            // Check if Docker CLI is installed
            let docker_installed = Command::new("which")
                .arg("docker")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);

            if !docker_installed {
                println!("{}", "❌ Docker CLI: Not installed".red());
                println!("{}", "  Run: craft docker install".blue());
                return Ok(());
            }

            println!("{}", "✅ Docker CLI: Installed".green());

            // Check if Colima is installed
            let colima_installed = Command::new("which")
                .arg("colima")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);

            if !colima_installed {
                println!("{}", "❌ Colima: Not installed".red());
                println!("{}", "  Run: craft docker install".blue());
                return Ok(());
            }

            println!("{}", "✅ Colima: Installed".green());

            // Check if Docker daemon is running
            let docker_running = Command::new("docker")
                .args(["info"])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);

            if docker_running {
                println!("{}", "✅ Docker daemon: Running".green());

                // Show Colima status
                let _ = Command::new("colima").arg("status").status();
            } else {
                println!("{}", "❌ Docker daemon: Not running".red());
                println!("{}", "  Run: craft docker start".blue());
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Provide status of rippled in Docker
    if let Ok(docker_manager) = docker::DockerManager::new() {
        match docker_manager.is_rippled_running().await {
            Ok(true) => {
                println!("{}", "✅ rippled is running in Docker container".green());
            }
            Ok(false) => {
                println!(
                    "{}",
                    "ℹ️  rippled is not currently running. To start it, run:".yellow()
                );
                println!("{}", "     craft start-rippled".blue());
            }
            Err(_) => {
                // Couldn't check rippled status
            }
        }
        println!();
    }

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
            Commands::Configure => {
                commands::configure().await?;
                println!("{}", "Configuration saved!".green());
            }
            Commands::ExportHex => {
                let config = commands::configure().await?;
                let wasm_path = commands::build(&config).await?;
                commands::copy_wasm_hex_to_clipboard(&wasm_path).await?;
            }
            Commands::Test { function } => {
                let config = commands::configure().await?;
                let wasm_path = commands::build(&config).await?;
                commands::test(&wasm_path, function).await?;
            }
            Commands::StartRippled { foreground } => {
                let docker_manager = docker::DockerManager::new()?;
                docker_manager.start_rippled(foreground).await?;
            }
            Commands::ListRippled => {
                let docker_manager = docker::DockerManager::new()?;
                docker_manager.list_containers().await?;
            }
            Commands::StopRippled => {
                let docker_manager = docker::DockerManager::new()?;
                docker_manager.stop_rippled().await?;
            }
            Commands::AdvanceLedger { count } => {
                let docker_manager = docker::DockerManager::new()?;
                docker_manager.advance_ledger(count).await?;
            }
            Commands::Docker { action } => {
                handle_docker_command(action).await?;
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
                "Start rippled" => {
                    let foreground = Confirm::new("Run rippled in foreground with console output? (Can be terminated with Ctrl+C)")
                        .with_default(true)
                        .prompt()?;

                    let docker_manager = docker::DockerManager::new()?;
                    docker_manager.start_rippled(foreground).await?;
                }
                "List rippled processes" => {
                    let docker_manager = docker::DockerManager::new()?;
                    docker_manager.list_containers().await?;
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
