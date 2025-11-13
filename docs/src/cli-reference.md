# CLI Reference

## Commands

### `analyze`

Run full analysis (complexity + clones + LOC).

```bash
mccabre analyze [OPTIONS] [PATH]
```

**Arguments:**

- `[PATH]` - Path to file or directory (default: `.`)

**Options:**

- `-j, --json` - Output in JSON format
- `--threshold <N>` - Complexity warning threshold
- `--min-tokens <N>` - Minimum tokens for clone detection (default: 30)
- `-c, --config <FILE>` - Path to config file
- `--no-gitignore` - Disable gitignore awareness

**Examples:**

```bash
# Analyze current directory
mccabre analyze

# Analyze specific path with JSON output
mccabre analyze ./src --json

# Custom thresholds
mccabre analyze ./src --threshold 15 --min-tokens 25
```

### `complexity`

Analyze cyclomatic complexity and LOC only.

```bash
mccabre complexity [OPTIONS] [PATH]
```

**Arguments:**

- `[PATH]` - Path to file or directory (default: `.`)

**Options:**

- `-j, --json` - Output in JSON format
- `--threshold <N>` - Complexity warning threshold
- `-c, --config <FILE>` - Path to config file
- `--no-gitignore` - Disable gitignore awareness

**Examples:**

```bash
# Check complexity of src/
mccabre complexity src/

# Fail if any file exceeds complexity of 20
mccabre complexity src/ --threshold 20
```

### `clones`

Detect code clones only.

```bash
mccabre clones [OPTIONS] [PATH]
```

**Arguments:**

- `[PATH]` - Path to file or directory (default: `.`)

**Options:**

- `-j, --json` - Output in JSON format
- `--min-tokens <N>` - Minimum tokens for detection (default: 30)
- `-c, --config <FILE>` - Path to config file
- `--no-gitignore` - Disable gitignore awareness

**Examples:**

```bash
# Find clones in current directory
mccabre clones .

# Find only large clones (50+ tokens)
mccabre clones . --min-tokens 50

# JSON output for processing
mccabre clones src/ --json | jq '.clones | length'
```

### `dump-config`

Display and optionally save current configuration.

```bash
mccabre dump-config [OPTIONS]
```

**Options:**

- `-c, --config <FILE>` - Path to config file (shows default if not specified)
- `-o, --output <PATH>` - Save configuration to file or directory

**Examples:**

```bash
# Show default configuration
mccabre dump-config

# Show loaded configuration
mccabre dump-config --config mccabre.toml

# Save default config to file
mccabre dump-config -o my-config.toml

# Save config to directory (creates mccabre.toml)
mccabre dump-config -o ./configs/

# Load and save to new location
mccabre dump-config -c old-config.toml -o new-config.toml
```

## Global Options

### `-h, --help`

Show help information.

```bash
mccabre --help
mccabre analyze --help
```

### `-V, --version`

Show version information.

```bash
mccabre --version
```

## Output Formats

### Terminal (Default)

Colored, human-readable output:

```text
FILE: src/main.rs
    Cyclomatic Complexity:   15 (warning)
    Physical LOC:            120
    Logical LOC:             85
```

Colors:

- **Green**: Low/good
- **Yellow**: Moderate/warning
- **Red**: High/error

### JSON

Machine-readable output for CI/CD:

```bash
mccabre analyze src/ --json
```

```json
{
  "files": [
    {
      "path": "src/main.rs",
      "loc": {
        "physical": 120,
        "logical": 85,
        "comments": 25,
        "blank": 10
      },
      "cyclomatic": {
        "file_complexity": 15,
        "functions": []
      }
    }
  ],
  "clones": [],
  "summary": {
    "total_files": 1,
    "total_physical_loc": 120,
    "total_logical_loc": 85,
    "avg_complexity": 15.0,
    "max_complexity": 15,
    "high_complexity_files": 1,
    "total_clones": 0
  }
}
```

## File Selection

### Supported Languages

- **Rust**: `.rs`
- **JavaScript**: `.js`, `.jsx`, `.mjs`, `.cjs`
- **TypeScript**: `.ts`, `.tsx`
- **Go**: `.go`
- **Java**: `.java`
- **C++**: `.cpp`, `.cc`, `.cxx`, `.h`, `.hpp`, `.hxx`

### Gitignore Support

Mccabre respects `.gitignore` files by default:

```bash
# Respects .gitignore (default)
mccabre analyze .

# Ignores .gitignore
mccabre analyze . --no-gitignore
```

Automatically skips:

- Files/directories in .gitignore
- `.git/` directory
- Binary files (by extension)

## Environment Variables

Currently none. Configuration via:

1. CLI flags (highest priority)
2. Config file
3. Defaults

## See Also

- [Configuration](./configuration.md)
- [Cyclomatic Complexity](./cyclomatic-complexity.md)
- [Clone Detection](./clone-detection.md)
- [Examples](./examples.md)
