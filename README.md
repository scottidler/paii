# PAII — Personal AI Infrastructure

A modular plugin system for Claude Code, designed for extensibility and team sharing.

Inspired by [Daniel Miessler's Kai/PAI system](https://github.com/danielmiessler/paii), reimplemented with a focus on true modularity, clean interfaces, and Rust performance.

## Features

- **Plugin System** — Install, manage, and create plugins in Python or Rust
- **Registry** — Discover and install plugins from remote registries
- **Hooks** — Intercept Claude Code events (security validation, history capture)
- **History** — File-based session tracking with YAML frontmatter
- **Config** — TOML-based configuration with environment variable support

## Installation

### From Source

```bash
git clone https://github.com/scottidler/paii.git
cd paii
cargo install --path .
```

### Shell Completions

```bash
# Zsh
mkdir -p ~/.zsh/completions
cp completions/_paii ~/.zsh/completions/
# Add to ~/.zshrc: fpath=(~/.zsh/completions $fpath)

# Bash
sudo cp completions/paii.bash /etc/bash_completion.d/paii

# Fish
cp completions/paii.fish ~/.config/fish/completions/
```

## Quick Start

```bash
# Initialize PAII in your home directory
paii init

# Check your setup
paii doctor

# Update registries
paii registry update

# Search for plugins
paii registry search hello

# Install a plugin from registry
paii plugin install hello-world

# List installed plugins
paii plugin list

# Run a plugin action
paii run hello-world greet World
```

## Commands

| Command | Description |
|---------|-------------|
| `paii init` | Initialize PAII configuration |
| `paii doctor` | Diagnose setup issues |
| `paii status` | Show system status |
| `paii plugin list` | List installed plugins |
| `paii plugin install <source>` | Install a plugin (path or registry name) |
| `paii plugin remove <name>` | Remove a plugin |
| `paii plugin new <name>` | Create a new plugin scaffold |
| `paii plugin info <name>` | Show plugin details |
| `paii registry list` | List configured registries |
| `paii registry update` | Update registry cache |
| `paii registry search <query>` | Search for plugins |
| `paii run <plugin> <action>` | Run a plugin action |
| `paii config show` | Show current configuration |
| `paii history recent` | Show recent history entries |

## Creating Plugins

```bash
# Create a Python plugin
paii plugin new my-skill --language python

# Create a Rust plugin
paii plugin new my-hook --language rust --type hook

# Install in dev mode (symlink)
paii plugin install --dev ./my-skill
```

### Plugin Structure

```
my-plugin/
├── plugin.toml      # Plugin manifest
├── SKILL.md         # Skill documentation (for skill plugins)
├── README.md        # Plugin README
├── pyproject.toml   # Python dependencies (or Cargo.toml for Rust)
└── src/
    └── main.py      # Entry point (or main.rs for Rust)
```

### plugin.toml

```toml
[plugin]
name = "my-plugin"
version = "0.1.0"
description = "My awesome plugin"
language = "python"

[hooks]
pre_tool_use = false
stop = true

[build]
type = "uv"
```

## Configuration

PAII looks for configuration in this order:
1. `--config` flag
2. `$PAII_CONFIG` environment variable
3. `$PAII_DIR/paii.toml`
4. `~/.config/paii/paii.toml`
5. `./paii.toml` (for development)

### Example paii.toml

```toml
[paii]
version = "0.1.0"

[paths]
plugins = "~/.config/paii/plugins"
history = "~/.config/paii/history"
registries = "~/.config/paii/registries"

[defaults]
language = "python"
log_level = "info"

[registries]
core = "https://raw.githubusercontent.com/scottidler/paii/main/registry/plugins.toml"

[hooks]
security_enabled = true
history_enabled = true
```

## Claude Code Integration

PAII integrates with Claude Code via hooks. Add to `.claude/settings.json`:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [{"type": "command", "command": "paii hook dispatch PreToolUse"}]
      }
    ],
    "Stop": [
      {
        "matcher": "*",
        "hooks": [{"type": "command", "command": "paii hook dispatch Stop"}]
      }
    ]
  }
}
```

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     Claude Code                          │
│                         │                                │
│    ┌────────────────────▼────────────────────┐          │
│    │              Hooks System               │          │
│    │  (PreToolUse, Stop, SessionStart, etc.) │          │
│    └────────────────────┬────────────────────┘          │
└─────────────────────────┼───────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                      paii CLI                            │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐       │
│  │ plugin  │ │registry │ │ history │ │  hook   │       │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘       │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                      Plugins                             │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐       │
│  │ Python/Rust │ │ Python/Rust │ │ Python/Rust │       │
│  │   Plugin    │ │   Plugin    │ │   Plugin    │       │
│  └─────────────┘ └─────────────┘ └─────────────┘       │
└─────────────────────────────────────────────────────────┘
```

## Development

```bash
# Run tests
cargo test

# Run with coverage
cargo llvm-cov --html

# Check formatting and lints
cargo fmt --check
cargo clippy -- -D warnings

# Build release binary
cargo build --release
```

## License

MIT

## Credits

- Inspired by [Daniel Miessler's Kai/PAI](https://github.com/danielmiessler/paii)
- Built on [Claude Code](https://claude.ai/code)

