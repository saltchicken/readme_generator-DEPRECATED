# README Generator

![License](https://img.shields.io/badge/license-MIT-blue.svg) ![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg) ![Status](https://img.shields.io/badge/status-active-green)

**README Generator** is a Rust-based CLI tool designed to streamline the documentation process for developers. By intelligently scanning your project's directory structure and source code, it constructs comprehensive, context-aware prompts specifically engineered for Large Language Models (LLMs). These prompts enable LLMs to generate high-quality, production-ready `README.md` files with minimal manual effort.

## Features

- **Automated Context Scanning:** Recursively scans directories to extract file structures and code contents using `code_context`.
- **Structured Prompt Engineering:** Generates prompts with defined roles, tasks, code context, and output requirements to ensure LLM consistency.
- **Customizable Output:**
    - **Style Control:** Define the tone of the documentation (e.g., "Professional", "Fun", "Minimalist").
    - **Focus Areas:** Add specific constraints or goals (e.g., "Focus on installation").
- **Flexible Input Methods:**
    - Supply project descriptions via CLI arguments.
    - Pipe descriptions via Stdin for workflow integration.
- **Smart Filtering:** Automatically ignores build artifacts (`target`, `node_modules`, `dist`) and hidden files to keep the context clean.

## Tech Stack

- **Language:** Rust (2021 Edition)
- **CLI Parsing:** `clap` (v4.5)
- **Error Handling:** `anyhow`
- **Context Generation:** `code_context` (Git dependency)

## Prerequisites

Before running this tool, ensure you have the following installed:

- **Rust & Cargo:** (Latest stable version recommended)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
  ```

## Installation

1. **Clone the repository:**
   ```bash
   git clone [https://github.com/yourusername/readme_generator.git](https://github.com/yourusername/readme_generator.git)
   cd readme_generator
   ```

2. **Build the release binary:**
   ```bash
   cargo build --release
   ```

3. **(Optional) Install to path:**
   ```bash
   cargo install --path .
   ```

## Usage

The tool is run via `cargo` or the compiled binary. It prints the generated prompt to standard output (`stdout`), which can be piped to a file or a clipboard.

### Basic Usage
Scan the current directory and generate a prompt using default settings:

```bash
cargo run -- --scan .
```

### Advanced Usage
Customize the style, description, and specific context constraints:

```bash
cargo run -- \
  --scan ./my-project \
  --style "Technical and Concise" \
  --description "A high-performance web scraper" \
  --context "Include a section on rate limiting configuration"
```

### Using Stdin
Pipe a description directly into the generator:

```bash
echo "A CLI for managing cloud resources" | cargo run -- --stdin --scan .
```

### CLI Options

| Flag | Short | Description | Default |
|------|-------|-------------|---------|
| `--scan <PATH>` | | Path to the project directory to scan for context. | None |
| `--style <TEXT>` | `-s` | Tone of the README (e.g., "Professional"). | "Professional and Concise" |
| `--description <TEXT>`| `-d` | The specific goal/summary of the project. | "Generate a README..." |
| `--context <TEXT>` | `-c` | Extra constraints or specific sections to include. | None |
| `--stdin` | | Read the description from standard input. | False |
| `--help` | `-h` | Print help information. | |

## Development

### Project Structure

```
src/
├── app/
│   ├── args.rs       # CLI argument parsing (Clap)
│   ├── mod.rs        # Main application logic and orchestration
│   ├── prompt.rs     # Prompt generation and formatting logic
│   └── scanner.rs    # Directory scanning via code_context
├── lib.rs            # Library entry point
└── main.rs           # Binary entry point
```

### Running Tests

```bash
cargo test
```

## License

This project is open source. Please see the [LICENSE](LICENSE) file for more details.

