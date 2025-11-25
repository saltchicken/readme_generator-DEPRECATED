use clap::Parser;
use std::path::PathBuf;


// Previously, this file incorrectly contained the logic from mod.rs.

/// CLI Argument structure
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The specific goal for the README (e.g., "Focus on installation steps")
    #[arg(short, long)]
    pub description: Option<String>,

    /// The style of the README (e.g., "Professional", "Fun", "Minimalist")
    #[arg(short, long, default_value = "Professional and Concise")]
    pub style: String,

    /// Extra constraints (e.g., "Include a section on contributing")
    #[arg(short, long)]
    pub context: Option<String>,

    /// Path to scan for code context (highly recommended for READMEs)
    #[arg(long)]
    pub scan: Option<PathBuf>,

    /// Read description from Stdin
    #[arg(long)]
    pub stdin: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}