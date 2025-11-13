# Installation

## From Source

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Build and Install

```bash
# Clone the repository
git clone https://github.com/yourusername/mccabre.git
cd mccabre

# Build and install
cargo install --path crates/cli
```

### Development Build

```bash
# Build in debug mode
cargo build

# Run directly
cargo run --bin mccabre -- analyze examples/

# Run tests
cargo test --quiet
```

## Verifying Installation

After building, verify your installation:

```bash
mccabre --version

mccabre --help

mccabre analyze examples/
```

You should see colored output with complexity metrics and detected clones.

## Uninstall

```bash
cargo uninstall mccabre
```

## Configuration

See [Configuration](./configuration.md) for customizing thresholds and behavior.
