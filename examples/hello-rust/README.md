# examples/hello-rust

A PAII skill plugin written in rust.

## Installation

```bash
paii plugin install --dev .
```

## Usage

```bash
paii run examples/hello-rust greet
paii run examples/hello-rust greet Alice
paii run examples/hello-rust version
```

## Development

```bash
cargo build --release
```

## License

MIT
