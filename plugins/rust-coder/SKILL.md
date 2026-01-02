---
name: rust-coder
description: Write Rust code using Scott's conventions. Use when creating Rust CLIs, libraries, reviewing Rust code, or when the user mentions Rust, cargo, or CLI tools.
---

# Rust Coding Conventions

This skill teaches you Scott's Rust coding patterns, tools, and conventions. Follow these guidelines when writing or reviewing Rust code.

## Creating New Rust CLI Projects

**Always use `scaffold` instead of `cargo new`** for CLI projects:

```bash
scaffold <project-name>
```

This sets up the complete project structure with all conventions pre-configured. Don't manually create Cargo.toml or add dependencies one by one.

### What scaffold provides

- **Cargo.toml** with correct edition (2024) and build.rs
- **src/main.rs** with logging setup and error handling
- **src/cli.rs** with clap derive pattern
- **src/config.rs** with YAML config loading
- **build.rs** for git describe versioning
- **.otto.yml** for CI/CD tasks
- Sample config file

## Core Dependencies

Use these crates for standard functionality:

| Purpose | Crate | Notes |
|---------|-------|-------|
| CLI parsing | `clap` | With `derive` feature |
| Error handling | `eyre` | NOT anyhow |
| Logging | `log` + `env_logger` | File output |
| Serialization | `serde` + `serde_yaml` | With `derive` feature |
| Terminal colors | `colored` | |
| Directory paths | `dirs` | For config/data dirs |

### Adding dependencies

Use `cargo add`, never hardcode versions in Cargo.toml:

```bash
cargo add clap --features derive
cargo add eyre
cargo add serde --features derive
cargo add serde_yaml
```

## Error Handling Pattern

Use `eyre::Result` and wrap errors with `.context()`:

```rust
use eyre::{Context, Result};

fn load_config(path: &Path) -> Result<Config> {
    let content = fs::read_to_string(path)
        .context("Failed to read config file")?;

    let config: Config = serde_yaml::from_str(&content)
        .context("Failed to parse config file")?;

    Ok(config)
}

fn main() -> Result<()> {
    let config = load_config(&path)
        .context("Configuration loading failed")?;
    Ok(())
}
```

**Never use `.unwrap()` in production code.** Use `?` with context.

## Logging Setup

Log to a file in `~/.local/share/<project>/logs/<project>.log`:

```rust
fn setup_logging() -> Result<()> {
    let log_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(env!("CARGO_PKG_NAME"))
        .join("logs");

    fs::create_dir_all(&log_dir)
        .context("Failed to create log directory")?;

    let log_file = log_dir.join(format!("{}.log", env!("CARGO_PKG_NAME")));

    let target = Box::new(
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)
            .context("Failed to open log file")?,
    );

    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Pipe(target))
        .init();

    Ok(())
}
```

## CLI Structure (clap derive)

Use the derive pattern with `#[command]` attributes:

```rust
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "myapp",
    about = "Description of what this tool does",
    version = env!("GIT_DESCRIBE"),
    after_help = "Logs are written to: ~/.local/share/myapp/logs/myapp.log"
)]
pub struct Cli {
    /// Path to config file
    #[arg(short, long, help = "Path to config file")]
    pub config: Option<PathBuf>,

    /// Enable verbose output
    #[arg(short, long, help = "Enable verbose output")]
    pub verbose: bool,
}
```

### Version from git describe

Use `GIT_DESCRIBE` env var set by build.rs, not `CARGO_PKG_VERSION`:

```rust
version = env!("GIT_DESCRIBE"),
```

## Configuration Pattern

Config files live at `~/.config/<project>/<project>.yml`:

```rust
impl Config {
    pub fn load(config_path: Option<&PathBuf>) -> Result<Self> {
        // Explicit path takes precedence
        if let Some(path) = config_path {
            return Self::load_from_file(path);
        }

        // Try ~/.config/<project>/<project>.yml
        if let Some(config_dir) = dirs::config_dir() {
            let primary = config_dir
                .join(env!("CARGO_PKG_NAME"))
                .join(format!("{}.yml", env!("CARGO_PKG_NAME")));
            if primary.exists() {
                return Self::load_from_file(&primary);
            }
        }

        // Try ./<project>.yml in current directory
        let local = PathBuf::from(format!("{}.yml", env!("CARGO_PKG_NAME")));
        if local.exists() {
            return Self::load_from_file(&local);
        }

        // Fall back to defaults
        Ok(Self::default())
    }
}
```

## Build Script (build.rs)

Always include build.rs for git describe versioning:

```rust
use std::process::Command;

fn main() {
    let git_describe = Command::new("git")
        .args(["describe", "--tags", "--always"])
        .output()
        .and_then(|output| {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                Err(std::io::Error::other("git describe failed"))
            }
        })
        .unwrap_or_else(|_| env!("CARGO_PKG_VERSION").to_string());

    println!("cargo:rustc-env=GIT_DESCRIBE={}", git_describe);
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/");
}
```

## CI/CD with Otto

Use `otto` for build tasks, not raw cargo commands:

```bash
otto ci      # Full CI: lint + check + test
otto check   # Compile + clippy + format check
otto test    # Run tests
otto cov     # Coverage report
otto build   # Release build
```

The scaffold tool generates `.otto.yml` automatically with standard tasks.

**For detailed otto configuration, see the `otto` skill** — it covers `.otto.yml` syntax, parameters, dependencies, and templates.

## Testing Patterns

### Use tempfile for filesystem tests

```rust
use tempfile::TempDir;

#[test]
fn test_creates_output_file() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output.txt");

    my_function(&output_path).unwrap();

    assert!(output_path.exists());
}
```

### Test helper functions

Create test utilities at the bottom of the file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> Config {
        Config::default()
    }

    fn create_test_cli(name: &str) -> Cli {
        Cli {
            config: None,
            verbose: false,
            // ... other fields
        }
    }

    #[test]
    fn test_something() {
        let config = create_test_config();
        // ...
    }
}
```

## Terminal Output

Use colored for user-facing output:

```rust
use colored::*;

// Success
println!("{} Operation completed", "✓".green());

// Info
println!("{} Processing...", "→".blue());

// Warning
println!("{} File already exists", "⚠".yellow());

// Error
eprintln!("{} Failed to connect", "✗".red());

// Styled text
println!("Created {}", filename.cyan().bold());
println!("Version: {}", version.dimmed());
```

## Code Style

- **Imports:** Group by std, external crates, then internal modules
- **Line length:** Keep under 100 characters
- **Comments:** Use `//` for inline, `///` for doc comments
- **Format:** Always run `cargo fmt` before committing

## Common Patterns

### Guard clauses

Exit early for invalid conditions:

```rust
fn process(input: &str) -> Result<Output> {
    if input.is_empty() {
        eyre::bail!("Input cannot be empty");
    }

    if !input.starts_with("valid") {
        eyre::bail!("Input must start with 'valid'");
    }

    // Main logic here
    Ok(output)
}
```

### Builder pattern for complex structs

Use builder pattern or Default + modifications for structs with many fields.

## What NOT to Do

- ❌ Don't use `anyhow` — use `eyre`
- ❌ Don't use `.unwrap()` in production code
- ❌ Don't hardcode dependency versions in Cargo.toml
- ❌ Don't use `cargo new` for CLIs — use `scaffold`
- ❌ Don't run raw `cargo` commands — use `otto`
- ❌ Don't use `println!` for errors — use `eprintln!`
- ❌ Don't skip error context — always use `.context()`

