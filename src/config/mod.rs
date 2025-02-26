use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub wasm_target: WasmTarget,
    pub build_mode: BuildMode,
    pub optimization_level: OptimizationLevel,
    pub use_wee_alloc: bool,
    pub project_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WasmTarget {
    UnknownUnknown,
    Wasip1,
}

impl std::fmt::Display for WasmTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WasmTarget::UnknownUnknown => write!(f, "wasm32-unknown-unknown"),
            WasmTarget::Wasip1 => write!(f, "wasm32-wasi-preview1"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BuildMode {
    Debug,
    Release,
}

impl std::fmt::Display for BuildMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildMode::Debug => write!(f, "debug"),
            BuildMode::Release => write!(f, "release"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OptimizationLevel {
    None,
    Small,
    Aggressive,
}

impl std::fmt::Display for OptimizationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptimizationLevel::None => write!(f, "none"),
            OptimizationLevel::Small => write!(f, "-Os"),
            OptimizationLevel::Aggressive => write!(f, "-Oz"),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            wasm_target: WasmTarget::UnknownUnknown,
            build_mode: BuildMode::Release,
            optimization_level: OptimizationLevel::Small,
            use_wee_alloc: false,
            project_path: std::env::current_dir().unwrap_or_default(),
        }
    }
} 