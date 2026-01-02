---
name: python-coder
description: Write Python code using Scott's conventions. Use when creating Python projects, packages, scripts, or when reviewing Python code. Uses uv, ruff, and modern tooling.
---

# Python Coding Conventions

This skill teaches you Scott's Python coding patterns, tools, and conventions. Follow these guidelines when writing or reviewing Python code.

## Package Manager: uv

**Always use `uv` instead of pip, poetry, or pipenv.**

```bash
# Create new project
uv init myproject
cd myproject

# Add dependencies
uv add requests click

# Add dev dependencies
uv add --dev pytest ruff mypy

# Run commands in the virtual environment
uv run python main.py
uv run pytest
uv run ruff check .
```

### Why uv?

- 10-100x faster than pip
- Built-in virtual environment management
- Lockfile support (uv.lock)
- Compatible with pyproject.toml

## Project Structure

```
myproject/
├── pyproject.toml      # Project metadata and dependencies
├── uv.lock             # Lockfile (committed to git)
├── .python-version     # Python version (e.g., "3.12")
├── .otto.yml           # CI/build tasks
├── src/
│   └── myproject/
│       ├── __init__.py
│       ├── main.py
│       └── utils.py
├── tests/
│   ├── __init__.py
│   ├── conftest.py
│   └── test_main.py
└── README.md
```

### pyproject.toml Template

```toml
[project]
name = "myproject"
version = "0.1.0"
description = "Description of the project"
readme = "README.md"
requires-python = ">=3.11"
dependencies = []

[project.optional-dependencies]
dev = [
    "pytest>=8.0",
    "pytest-cov>=4.0",
    "ruff>=0.4",
    "mypy>=1.10",
]

[project.scripts]
myproject = "myproject.main:main"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.ruff]
line-length = 100
target-version = "py311"

[tool.ruff.lint]
select = ["E", "F", "I", "UP", "B", "SIM"]

[tool.mypy]
python_version = "3.11"
strict = true

[tool.pytest.ini_options]
testpaths = ["tests"]
addopts = "-v"
```

## Creating New Projects

```bash
# Initialize with uv
uv init myproject
cd myproject

# Set Python version
echo "3.12" > .python-version

# Add development dependencies
uv add --dev pytest pytest-cov ruff mypy

# Create source structure
mkdir -p src/myproject tests
touch src/myproject/__init__.py
touch src/myproject/main.py
touch tests/__init__.py
touch tests/conftest.py
```

## Linting and Formatting: ruff

**Use `ruff` for both linting and formatting.** It replaces flake8, isort, and black.

```bash
# Check for issues
uv run ruff check .

# Fix auto-fixable issues
uv run ruff check --fix .

# Format code
uv run ruff format .

# Check formatting without changes
uv run ruff format --check .
```

### ruff Configuration

In `pyproject.toml`:

```toml
[tool.ruff]
line-length = 100
target-version = "py311"

[tool.ruff.lint]
select = [
    "E",    # pycodestyle errors
    "F",    # pyflakes
    "I",    # isort
    "UP",   # pyupgrade
    "B",    # bugbear
    "SIM",  # simplify
]
ignore = ["E501"]  # line too long (handled by formatter)

[tool.ruff.format]
quote-style = "double"
indent-style = "space"
```

## Type Hints: mypy

**Always use type hints.** Run mypy in strict mode:

```bash
uv run mypy src/
```

### Type Hint Patterns

```python
from typing import Optional
from collections.abc import Sequence

def process_items(
    items: Sequence[str],
    prefix: str = "",
    limit: int | None = None,
) -> list[str]:
    """Process items with optional prefix and limit."""
    result = [f"{prefix}{item}" for item in items]
    if limit is not None:
        result = result[:limit]
    return result

def find_user(user_id: int) -> Optional[dict[str, str]]:
    """Find user by ID, returns None if not found."""
    ...
```

**Type hint rules:**
- Use `|` for unions (Python 3.10+): `str | None` not `Optional[str]`
- Use `list`, `dict`, `set` directly (Python 3.9+), not `List`, `Dict`, `Set`
- Use `collections.abc` for abstract types: `Sequence`, `Mapping`, `Iterable`

## Testing: pytest

```bash
# Run all tests
uv run pytest

# Run with coverage
uv run pytest --cov=src --cov-report=html

# Run specific test
uv run pytest tests/test_main.py::test_specific_function

# Run with verbose output
uv run pytest -v
```

### Test File Structure

```python
# tests/conftest.py
import pytest

@pytest.fixture
def sample_data() -> dict[str, str]:
    return {"key": "value"}

# tests/test_main.py
from myproject.main import process

def test_process_returns_expected(sample_data: dict[str, str]) -> None:
    result = process(sample_data)
    assert result == expected

def test_process_handles_empty() -> None:
    result = process({})
    assert result == []
```

## CI/CD with Otto

Use `otto` for build tasks. **For detailed otto configuration, see the `otto` skill.**

```bash
otto ci        # Full CI: lint + format + typecheck + test
otto test      # Run pytest
otto cov       # Coverage report
```

### Python .otto.yml Template

```yaml
otto:
  api: 1
  tasks: [ci]

tasks:
  lint:
    help: "Run ruff linter"
    bash: uv run ruff check .

  format:
    help: "Check formatting"
    bash: uv run ruff format --check .

  typecheck:
    help: "Run mypy"
    bash: uv run mypy src/

  test:
    help: "Run pytest"
    bash: uv run pytest

  cov:
    help: "Coverage report"
    bash: |
      uv run pytest --cov=src --cov-report=html
      echo "📄 Report: htmlcov/index.html"

  ci:
    help: "Full CI"
    before: [lint, format, typecheck, test]
    bash: echo "✅ All CI checks passed!"

  clean:
    help: "Clean artifacts"
    bash: |
      rm -rf .pytest_cache .mypy_cache .ruff_cache
      rm -rf htmlcov .coverage
      rm -rf dist build *.egg-info
      find . -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true
      echo "✅ Cleaned"
```

## CLI Applications: click

Use `click` for CLI applications (not argparse):

```python
import click

@click.command()
@click.option("--name", "-n", required=True, help="Name to greet")
@click.option("--count", "-c", default=1, help="Number of greetings")
@click.option("--verbose", "-v", is_flag=True, help="Verbose output")
def main(name: str, count: int, verbose: bool) -> None:
    """Greet someone multiple times."""
    for _ in range(count):
        if verbose:
            click.echo(f"Greeting {name}...")
        click.echo(f"Hello, {name}!")

if __name__ == "__main__":
    main()
```

### Entry Points

In `pyproject.toml`:

```toml
[project.scripts]
myapp = "myproject.main:main"
```

After install: `uv run myapp --name World`

## Error Handling

Use explicit exception handling with context:

```python
from pathlib import Path

class ConfigError(Exception):
    """Configuration-related errors."""
    pass

def load_config(path: Path) -> dict:
    """Load configuration from file."""
    if not path.exists():
        raise ConfigError(f"Config file not found: {path}")

    try:
        content = path.read_text()
    except PermissionError as e:
        raise ConfigError(f"Cannot read config file: {path}") from e

    try:
        return yaml.safe_load(content)
    except yaml.YAMLError as e:
        raise ConfigError(f"Invalid YAML in config file: {path}") from e
```

**Error handling rules:**
- Create specific exception classes for your domain
- Use `raise ... from e` to chain exceptions
- Catch specific exceptions, not bare `except:`
- Include context in error messages

## Logging

Use the standard `logging` module:

```python
import logging
from pathlib import Path

def setup_logging(log_dir: Path | None = None, debug: bool = False) -> None:
    """Configure logging with file and console output."""
    level = logging.DEBUG if debug else logging.INFO

    handlers: list[logging.Handler] = [logging.StreamHandler()]

    if log_dir:
        log_dir.mkdir(parents=True, exist_ok=True)
        file_handler = logging.FileHandler(log_dir / "app.log")
        handlers.append(file_handler)

    logging.basicConfig(
        level=level,
        format="%(asctime)s - %(name)s - %(levelname)s - %(message)s",
        handlers=handlers,
    )

logger = logging.getLogger(__name__)

def process():
    logger.info("Starting process")
    logger.debug("Debug details here")
```

## Code Style

### Imports

Group and order imports:

```python
# Standard library
import os
from pathlib import Path

# Third-party
import click
import yaml

# Local
from myproject.utils import helper
```

ruff handles this automatically with `I` rule.

### Docstrings

Use Google-style docstrings:

```python
def process_data(data: list[dict], limit: int | None = None) -> list[str]:
    """Process data items and return formatted strings.

    Args:
        data: List of data dictionaries to process.
        limit: Maximum number of items to process. None means no limit.

    Returns:
        List of formatted string representations.

    Raises:
        ValueError: If data contains invalid items.
    """
    ...
```

### Naming Conventions

- `snake_case` for functions, variables, modules
- `PascalCase` for classes
- `UPPER_SNAKE_CASE` for constants
- `_private` prefix for internal functions/variables

## What NOT to Do

- ❌ Don't use pip directly — use `uv`
- ❌ Don't use poetry/pipenv — use `uv`
- ❌ Don't use black/isort/flake8 separately — use `ruff`
- ❌ Don't use argparse for CLIs — use `click`
- ❌ Don't use `Optional[X]` — use `X | None`
- ❌ Don't use `List`, `Dict` from typing — use `list`, `dict`
- ❌ Don't use bare `except:` — catch specific exceptions
- ❌ Don't skip type hints — always annotate
- ❌ Don't run raw pytest/ruff — use `uv run` or `otto`

