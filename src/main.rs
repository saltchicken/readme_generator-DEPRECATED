use anyhow::Result;
use readme_generator::app;

// ‼️ Logic moved to src/app/mod.rs to satisfy Requirement #6 (Entry Point Structure)
fn main() -> Result<()> {
    // We delegate immediately to the application layer.
    // Error handling is bubbled up to here and printed nicely by anyhow.
    app::run()
}
