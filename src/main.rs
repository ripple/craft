mod commands;
mod config;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
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
    /// Export WASM as hex
    ExportHex,
    /// Setup wee_alloc for smaller binary size
    SetupWeeAlloc,
    /// Test a WASM smart contract
    Test {
        /// Function to test (default: get_greeting)
        #[arg(short, long)]
        function: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(cmd) => match cmd {
            Commands::Build => {
                let config = commands::configure().await?;
                let wasm_path = commands::build(&config).await?;
                
                if !matches!(config.optimization_level, config::OptimizationLevel::None) {
                    commands::optimize(&wasm_path, &config.optimization_level).await?;
                }

                if config.use_wee_alloc {
                    commands::setup_wee_alloc(&config.project_path).await?;
                }

                let choices = vec![
                    "Export as hex",
                    "Test contract",
                    "Exit",
                ];

                match Select::new("What would you like to do next?", choices).prompt()? {
                    "Export as hex" => commands::export_hex(&wasm_path).await?,
                    "Test contract" => commands::test(&wasm_path, None).await?,
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
                commands::export_hex(&wasm_path).await?;
            }
            Commands::SetupWeeAlloc => {
                commands::setup_wee_alloc(&std::env::current_dir()?).await?;
            }
            Commands::Test { function } => {
                let config = commands::configure().await?;
                let wasm_path = commands::build(&config).await?;
                commands::test(&wasm_path, function).await?;
            }
        },
        None => {
            let choices = vec![
                "Build WASM module",
                "Configure settings",
                "Export WASM as hex",
                "Test contract",
                "Setup wee_alloc",
                "Exit",
            ];

            match Select::new("What would you like to do?", choices).prompt()? {
                "Build WASM module" => {
                    let config = commands::configure().await?;
                    let wasm_path = commands::build(&config).await?;
                    
                    if !matches!(config.optimization_level, config::OptimizationLevel::None) {
                        commands::optimize(&wasm_path, &config.optimization_level).await?;
                    }

                    if config.use_wee_alloc {
                        commands::setup_wee_alloc(&config.project_path).await?;
                    }
                }
                "Configure settings" => {
                    commands::configure().await?;
                    println!("{}", "Configuration saved!".green());
                }
                "Export WASM as hex" => {
                    let config = commands::configure().await?;
                    let wasm_path = commands::build(&config).await?;
                    commands::export_hex(&wasm_path).await?;
                }
                "Test contract" => {
                    let config = commands::configure().await?;
                    let wasm_path = commands::build(&config).await?;
                    commands::test(&wasm_path, None).await?;
                }
                "Setup wee_alloc" => {
                    commands::setup_wee_alloc(&std::env::current_dir()?).await?;
                }
                _ => (),
            }
        }
    }

    Ok(())
} 