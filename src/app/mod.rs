pub mod args;
pub mod prompt;
pub mod scanner;

use anyhow::Result;
use args::parse_args;
use prompt::generate_readme_prompt;
use scanner::scan_repository;
use std::io::{self, Read};

/// Main application entry point.
/// ‼️ Orchestrates the flow of data between modules.
pub fn run() -> Result<()> {
    let args = parse_args();

    // 1. Handle Input (Stdin vs Args)
    let project_description = resolve_description(args.description, args.stdin)?;

    // 2. Scan Context (if path provided)
    // ‼️ Leverages code_context to get actual file contents for the LLM to read
    let repo_context = if let Some(path) = args.scan {
        scan_repository(&path)?
    } else {
        String::new()
    };

    // 3. Generate the Prompt
    let final_output = generate_readme_prompt(
        &project_description,
        &args.style,
        &args.context.unwrap_or_default(),
        &repo_context,
    );

    // 4. Output
    println!("{}", final_output);

    Ok(())
}

/// Helper to handle stdin vs argument logic
/// ‼️ Refactoring Strategy (Req #7): Extracted logic for cleanliness.
fn resolve_description(desc_arg: Option<String>, use_stdin: bool) -> Result<String> {
    if use_stdin {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer.trim().to_string())
    } else {
        Ok(desc_arg.unwrap_or_else(|| "Generate a README for this project.".to_string()))
    }
}
