# Configuration

Mccabre can be configured via config files and CLI flags.

## Configuration Priority

1. **CLI flags** (highest priority)
2. **Config file** (`mccabre.toml`)
3. **Defaults** (lowest priority)

CLI flags override config file settings.

## Config File

Create a `mccabre.toml` file in your project root:

```toml
[complexity]
warning_threshold = 10
error_threshold = 20

[clones]
enabled = true
min_tokens = 30

[files]
respect_gitignore = true
```

### Generating a Config File

You can generate a config file using the `dump-config` command:

```bash
# Save default config to current directory
mccabre dump-config -o mccabre.toml

# Save to a specific location
mccabre dump-config -o /path/to/config.toml

# Save to a directory (creates mccabre.toml in that directory)
mccabre dump-config -o ./configs/

# Load existing config and save to new location
mccabre dump-config -c old-config.toml -o new-config.toml
```

This is useful for:

- Creating a starting point for customization
- Copying configurations between projects
- Version controlling your settings

## Configuration Options

### Complexity Settings

```toml
[complexity]
warning_threshold = 10    # Yellow warning at this level
error_threshold = 20      # Red error at this level
```

**Defaults:**

- `warning_threshold`: 10
- `error_threshold`: 20

**CLI Override:**

```bash
mccabre analyze --threshold 15
```

### Clone Detection Settings

```toml
[clones]
enabled = true      # Enable/disable clone detection
min_tokens = 30     # Minimum token sequence length
```

**Defaults:**

- `enabled`: true
- `min_tokens`: 30

**CLI Override:**

```bash
mccabre analyze --min-tokens 25
```

### File Settings

```toml
[files]
respect_gitignore = true  # Honor .gitignore files
```

**Defaults:**

- `respect_gitignore`: true

**CLI Override:**

```bash
mccabre analyze --no-gitignore
```

## Loading Configuration

### Automatic Discovery

Mccabre searches for config files in this order:

1. `mccabre.toml`
2. `.mccabre.toml`
3. `.mccabre/config.toml`

The first file found is used.

### Explicit Path

Specify a config file:

```bash
mccabre analyze --config /path/to/config.toml
```

### No Config File

If no config file exists, defaults are used.

## Example Configurations

### Strict Mode

For critical codebases:

```toml
[complexity]
warning_threshold = 5
error_threshold = 10

[clones]
enabled = true
min_tokens = 20

[files]
respect_gitignore = true
```

### Lenient Mode

For legacy codebases:

```toml
[complexity]
warning_threshold = 20
error_threshold = 40

[clones]
enabled = true
min_tokens = 50

[files]
respect_gitignore = true
```

### Clone-Focused

Focus on duplication:

```toml
[complexity]
warning_threshold = 100  # Effectively disable
error_threshold = 200

[clones]
enabled = true
min_tokens = 15  # Very sensitive

[files]
respect_gitignore = true
```

## Per-Project Settings

Different projects can have different configs:

```bash
project-a/
  ├── mccabre.toml  # Strict settings
  └── src/

project-b/
  ├── mccabre.toml  # Lenient settings
  └── src/
```

Each project's config is automatically loaded when analyzing that directory.

## Viewing Current Configuration

Check what settings are active:

```bash
mccabre dump-config
```

Output:

```text
CONFIGURATION
================================================================================

Complexity Settings:
  Warning threshold:     10
  Error threshold:       20

Clone Detection Settings:
  Enabled:               true
  Minimum tokens:        30

File Settings:
  Respect .gitignore:    true
```

## Ignoring Files

Use `.gitignore` to exclude files/directories:

```text
# .gitignore
target/
node_modules/
build/
*.generated.rs
```

Mccabre automatically respects these exclusions.

To analyze everything (ignore gitignore):

```bash
mccabre analyze . --no-gitignore
```

## See Also

- [CLI Reference](./cli-reference.md)
- [Cyclomatic Complexity](./cyclomatic-complexity.md)
- [Clone Detection](./clone-detection.md)
