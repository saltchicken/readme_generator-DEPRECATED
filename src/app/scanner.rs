use anyhow::{Context, Result};
use code_context::app::{generate, models::RuntimeConfig};
use std::path::Path;

/// Scans the directory using the existing `code_context` dependency.

pub fn scan_repository(path: &Path) -> Result<String> {
    eprintln!("🔍 Scanning directory for README context: {:?}", path);

    // Configure the scanner
    let config = RuntimeConfig {
        include: vec!["**/*".to_string()],
        exclude: vec![
            "**/target/**".to_string(),
            "**/.git/**".to_string(),
            "**/node_modules/**".to_string(),
            "**/dist/**".to_string(),
        ],
        include_in_tree: vec![],
        tree_only_output: false,
    };

    // Execute scan
    generate(config, path.to_path_buf())
        .map_err(|e| anyhow::anyhow!(e))
        .context("Failed to generate code context from directory")
}