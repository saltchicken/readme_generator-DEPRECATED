use anyhow::Result;
use readme_generator::app;


fn main() -> Result<()> {
    // We delegate immediately to the application layer.
    // Error handling is bubbled up to here and printed nicely by anyhow.
    app::run()
}