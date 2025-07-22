mod commands;
mod config;
mod docker;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use inquire::Confirm;
use inquire::Select;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Clone, ValueEnum)]
enum ListResource {
    /// List all projects in the projects/ directory
    Projects,
    /// List all available test cases
    Tests,
    /// List all test fixtures
    Fixtures,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Build a WASM module
    Build {
        /// Project name under projects directory
        project: Option<String>,
        /// Build in release mode (default for WASM)
        #[arg(short, long)]
        release: bool,
        /// Build in debug mode
        #[arg(short, long, conflicts_with = "release")]
        debug: bool,
        /// Optimization level (none, small, aggressive)
        #[arg(short = 'O', long, value_enum)]
        opt: Option<config::OptimizationLevel>,
        /// Run cargo fmt after building
        #[arg(long)]
        fmt: bool,
        /// Additional arguments to pass to cargo
        #[arg(last = true)]
        cargo_args: Vec<String>,
    },
    /// List available projects, tests, or other resources
    List {
        /// What to list
        #[arg(value_enum)]
        resource: ListResource,
    },
    /// Configure build settings
    Configure,
    /// Export WASM as hex
    ExportHex,
    /// Test a WASM smart contract
    Test {
        /// Project name to test
        project: Option<String>,
        /// Test case to run (defaults to 'success')
        #[arg(short, long)]
        case: Option<String>,
        /// Run all test cases
        #[arg(long, conflicts_with = "case")]
        all: bool,
        /// Function to test (defaults to 'finish')
        #[arg(short, long)]
        function: Option<String>,
        /// Build before testing
        #[arg(long, default_value_t = true)]
        build: bool,
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
        /// List available test cases
        #[arg(long)]
        list: bool,
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
    // Docker {
    //     #[command(subcommand)]
    //     action: Option<DockerAction>,
    // },
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

// async fn handle_docker_command(action: Option<DockerAction>) -> Result<()> {
//     use std::process::Command;
//
//     match action {
//         Some(DockerAction::Install) => {
//             println!(
//                 "{}",
//                 "Installing Colima (lightweight Docker runtime)...".cyan()
//             );
//
//             // Check if Homebrew is installed
//             let brew_check = Command::new("which")
//                 .arg("brew")
//                 .output()
//                 .map(|o| o.status.success())
//                 .unwrap_or(false);
//
//             if !brew_check {
//                 anyhow::bail!(
//                     "Homebrew is not installed. Please install it first:\n\
//                     /bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
//                 );
//             }
//
//             // Install Colima and Docker CLI
//             let status = Command::new("brew")
//                 .args(["install", "colima", "docker"])
//                 .status()?;
//
//             if status.success() {
//                 println!("{}", "Colima installed successfully!".green());
//                 println!("{}", "Run 'craft docker start' to start Colima.".blue());
//             } else {
//                 anyhow::bail!("Failed to install Colima");
//             }
//         }
//         Some(DockerAction::Start) => {
//             println!("{}", "Starting Colima...".cyan());
//             let status = Command::new("colima").arg("start").status()?;
//
//             if status.success() {
//                 println!("{}", "Colima started successfully!".green());
//             } else {
//                 anyhow::bail!("Failed to start Colima");
//             }
//         }
//         Some(DockerAction::Stop) => {
//             println!("{}", "Stopping Colima...".cyan());
//             let status = Command::new("colima").arg("stop").status()?;
//
//             if status.success() {
//                 println!("{}", "Colima stopped successfully!".green());
//             } else {
//                 anyhow::bail!("Failed to stop Colima");
//             }
//         }
//         Some(DockerAction::Status) | None => {
//             // Check Docker status
//             println!("{}", "Checking Docker status...".cyan());
//
//             // Check if Docker CLI is installed
//             let docker_installed = Command::new("which")
//                 .arg("docker")
//                 .output()
//                 .map(|o| o.status.success())
//                 .unwrap_or(false);
//
//             if !docker_installed {
//                 println!("{}", "❌ Docker CLI: Not installed".red());
//                 println!("{}", "  Run: craft docker install".blue());
//                 return Ok(());
//             }
//
//             println!("{}", "✅ Docker CLI: Installed".green());
//
//             // Check if Colima is installed
//             let colima_installed = Command::new("which")
//                 .arg("colima")
//                 .output()
//                 .map(|o| o.status.success())
//                 .unwrap_or(false);
//
//             if !colima_installed {
//                 println!("{}", "❌ Colima: Not installed".red());
//                 println!("{}", "  Run: craft docker install".blue());
//                 return Ok(());
//             }
//
//             println!("{}", "✅ Colima: Installed".green());
//
//             // Check if Docker daemon is running
//             let docker_running = Command::new("docker")
//                 .args(["info"])
//                 .output()
//                 .map(|o| o.status.success())
//                 .unwrap_or(false);
//
//             if docker_running {
//                 println!("{}", "✅ Docker daemon: Running".green());
//
//                 // Show Colima status
//                 let _ = Command::new("colima").arg("status").status();
//             } else {
//                 println!("{}", "❌ Docker daemon: Not running".red());
//                 println!("{}", "  Run: craft docker start".blue());
//             }
//         }
//     }
//
//     Ok(())
// }

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
            Commands::Build {
                project,
                release: _,
                debug,
                opt,
                fmt,
                cargo_args,
            } => {
                // Determine build mode - default to release for WASM
                let build_mode = if debug {
                    config::BuildMode::Debug
                } else {
                    config::BuildMode::Release
                };

                let project_path = if let Some(proj) = project {
                    std::env::current_dir()?.join("projects").join(proj)
                } else if atty::is(atty::Stream::Stdout) {
                    // Interactive selection if TTY available
                    let config = commands::configure().await?;
                    commands::build(&config).await?;
                    if fmt {
                        utils::run_cargo_fmt()?;
                    }
                    return Ok(());
                } else {
                    // Non-interactive mode - list projects and exit
                    println!("{}", "No project specified in non-interactive mode.".red());
                    println!();
                    commands::list_projects()?;
                    println!();
                    println!("{}", "To build a project, use one of:".yellow());
                    println!("  craft build <project-name>    # Build specific project");
                    println!("  craft build --help            # Show all options");
                    anyhow::bail!("No project specified");
                };

                // Prepare configuration
                let config = config::Config {
                    project_path,
                    build_mode,
                    optimization_level: opt.unwrap_or(config::OptimizationLevel::Small),
                    ..Default::default()
                };

                // Execute build
                let wasm_path = commands::build_with_args(&config, &cargo_args).await?;

                // Run formatter if requested
                if fmt {
                    utils::run_cargo_fmt()?;
                }

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
            Commands::List { resource } => match resource {
                ListResource::Projects => {
                    commands::list_projects()?;
                }
                ListResource::Tests => {
                    commands::list_all_tests()?;
                }
                ListResource::Fixtures => {
                    commands::list_fixtures()?;
                }
            },
            Commands::Configure => {
                commands::configure().await?;
                println!("{}", "Configuration saved!".green());
            }
            Commands::ExportHex => {
                let config = commands::configure().await?;
                let wasm_path = commands::build(&config).await?;
                commands::copy_wasm_hex_to_clipboard(&wasm_path).await?;
            }
            Commands::Test {
                project,
                case,
                all,
                function,
                build,
                verbose,
                list,
            } => {
                // Handle list mode
                if list {
                    commands::list_test_cases(project.as_deref())?;
                    return Ok(());
                }

                let project_name = if let Some(proj) = project {
                    proj
                } else if atty::is(atty::Stream::Stdout) {
                    // Interactive mode
                    let config = commands::configure().await?;
                    let wasm_path = if build {
                        commands::build(&config).await?
                    } else {
                        utils::find_wasm_output(&config.project_path)?
                    };
                    commands::test(&wasm_path, function).await?;
                    return Ok(());
                } else {
                    // Non-interactive mode
                    println!("{}", "No project specified in non-interactive mode.".red());
                    println!();
                    commands::list_projects()?;
                    println!();
                    println!("{}", "To test a project, use one of:".yellow());
                    println!(
                        "  craft test <project-name>              # Test with default 'success' case"
                    );
                    println!("  craft test <project-name> --case all   # Run all test cases");
                    println!(
                        "  craft test <project-name> --list       # List available test cases"
                    );
                    println!("  craft test --help                      # Show all options");
                    anyhow::bail!("No project specified");
                };

                // Build if requested
                let wasm_path = if build {
                    let project_path = std::env::current_dir()?
                        .join("projects")
                        .join(&project_name);
                    let config = config::Config {
                        project_path,
                        build_mode: config::BuildMode::Release,
                        optimization_level: config::OptimizationLevel::Small,
                        ..Default::default()
                    };
                    commands::build(&config).await?
                } else {
                    let project_path = std::env::current_dir()?
                        .join("projects")
                        .join(&project_name);
                    utils::find_wasm_output(&project_path)?
                };

                // Determine test cases to run
                let test_cases = if all {
                    commands::discover_test_cases(&project_name)?
                } else if let Some(case_name) = case {
                    vec![case_name]
                } else {
                    vec!["success".to_string()] // default
                };

                // Run tests
                for test_case in test_cases {
                    commands::run_test(&wasm_path, &test_case, function.as_deref(), verbose)?;
                }
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
            // Commands::Docker { action } => {
            //     handle_docker_command(action).await?;
            // }
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
        let cli = Cli::parse_from(["craft", "build", "myproj", "--debug", "--opt", "none"]);
        match cli.command {
            Some(Commands::Build {
                project,
                debug,
                opt,
                ..
            }) => {
                assert_eq!(project.unwrap(), "myproj");
                assert!(debug);
                assert_eq!(opt.unwrap(), OptimizationLevel::None);
            }
            other => panic!("Expected Build command, got: {other:?}"),
        }
    }

    #[test]
    fn test_build_defaults() {
        let cli = Cli::parse_from(["craft", "build"]);
        match cli.command {
            Some(Commands::Build {
                project,
                release,
                debug,
                opt,
                ..
            }) => {
                assert!(project.is_none());
                assert!(!release);
                assert!(!debug); // defaults to release mode for WASM
                assert!(opt.is_none());
            }
            other => panic!("Expected Build command, got: {other:?}"),
        }
    }

    #[test]
    fn test_build_with_positional_project() {
        let cli = Cli::parse_from(["craft", "build", "myproj", "--debug"]);
        match cli.command {
            Some(Commands::Build { project, debug, .. }) => {
                assert_eq!(project.unwrap(), "myproj");
                assert!(debug);
            }
            other => panic!("Expected Build command, got: {other:?}"),
        }
    }
}
