# Example Usage

This page demonstrates common Mccabre workflows with real examples.

## Example Files

The `examples/` directory contains sample files demonstrating different issues:

- **complex.js** - High cyclomatic complexity
- **long.py** - Many lines of code with comments
- **not_dry.go** - Duplicated code (clones)

## Analyzing the Examples

### Full Analysis

```bash
mccabre analyze examples/
```

**Output:**

```text
================================================================================
MCCABRE CODE ANALYSIS REPORT
================================================================================

SUMMARY
--------------------------------------------------------------------------------
Total files analyzed:        3
Total physical LOC:          215
Total logical LOC:           165
Average complexity:          10.33
Maximum complexity:          18
High complexity files:       1
Clone groups detected:       2

FILE METRICS
--------------------------------------------------------------------------------
FILE: examples/complex.js
    Cyclomatic Complexity:   18 (moderate)
    Physical LOC:            49
    Logical LOC:             42
    Comment lines:           2
    Blank lines:             5

FILE: examples/long.py
    Cyclomatic Complexity:   8 (low)
    Physical LOC:            95
    Logical LOC:             62
    Comment lines:           18
    Blank lines:             15

FILE: examples/not_dry.go
    Cyclomatic Complexity:   6 (low)
    Physical LOC:            71
    Logical LOC:             61
    Comment lines:           5
    Blank lines:             5

DETECTED CLONES
--------------------------------------------------------------------------------
Clone Group #1 (length: 30 tokens, 3 occurrences)
  - examples/not_dry.go:12-26
  - examples/not_dry.go:30-44
  - examples/not_dry.go:48-62
```

### Complexity Only

```bash
mccabre complexity examples/complex.js
```

Shows that `complex.js` has high cyclomatic complexity due to many conditional branches.

### Clone Detection Only

```bash
mccabre clones examples/not_dry.go
```

Identifies the three nearly-identical functions in `not_dry.go`.

## Real-World Scenarios

### Scenario 1: Pre-Commit Check

Ensure code quality before committing:

```bash
#!/bin/sh
# .git/hooks/pre-commit

echo "Checking code complexity..."

# Get list of staged Rust files
FILES=$(git diff --cached --name-only --diff-filter=ACM | grep '\.rs$')

if [ -n "$FILES" ]; then
    mccabre complexity $FILES --threshold 15
    if [ $? -ne 0 ]; then
        echo "❌ Complexity check failed!"
        exit 1
    fi
fi

echo "✅ Complexity check passed!"
```

### Scenario 2: Finding Refactoring Targets

Combine complexity and clone detection:

```bash
# Find high-complexity files
echo "=== High Complexity Files ==="
mccabre complexity src/ --json | \
  jq -r '.files[] | select(.cyclomatic.file_complexity > 15) | .path'

# Find duplicated code
echo "\n=== Code Clones ==="
mccabre clones src/ --min-tokens 25
```

Then refactor the flagged files.

### Scenario 3: Tracking Technical Debt

Weekly complexity tracking:

```bash
#!/bin/bash
# weekly-report.sh

DATE=$(date +%Y-%m-%d)
REPORT_DIR="reports"

mkdir -p "$REPORT_DIR"

# Generate report
mccabre analyze src/ --json > "$REPORT_DIR/report-$DATE.json"

# Extract key metrics
echo "Complexity Report - $DATE"
jq '.summary' "$REPORT_DIR/report-$DATE.json"

# Compare with last week
LAST_WEEK=$(ls -t $REPORT_DIR/report-*.json | sed -n 2p)
if [ -n "$LAST_WEEK" ]; then
    echo "\nChange from last week:"
    jq -s '.[1].summary.avg_complexity - .[0].summary.avg_complexity' \
      "$LAST_WEEK" "$REPORT_DIR/report-$DATE.json"
fi
```

### Scenario 5: Code Review Automation

Automatically comment on PRs with complexity issues:

```bash
#!/bin/bash
# pr-comment.sh

# Run analysis
mccabre analyze src/ --json > complexity.json

# Extract high complexity files
HIGH_COMPLEXITY=$(jq -r '.files[] | select(.cyclomatic.file_complexity > 15) |
  "- `\(.path)`: Complexity \(.cyclomatic.file_complexity)"' complexity.json)

if [ -n "$HIGH_COMPLEXITY" ]; then
    # Post comment to GitHub PR (requires gh CLI)
    gh pr comment --body "## ⚠️ Complexity Warning

Files with high complexity:
$HIGH_COMPLEXITY

Consider refactoring before merging."
fi
```

### Scenario 5: Compare Branches

Compare complexity between branches:

```bash
#!/bin/bash
# compare-branches.sh

MAIN_BRANCH="main"
FEATURE_BRANCH=$(git branch --show-current)

# Analyze main
git checkout "$MAIN_BRANCH"
mccabre analyze src/ --json > /tmp/main-complexity.json

# Analyze feature
git checkout "$FEATURE_BRANCH"
mccabre analyze src/ --json > /tmp/feature-complexity.json

# Compare
echo "=== Complexity Comparison ==="
echo "Main branch avg: $(jq '.summary.avg_complexity' /tmp/main-complexity.json)"
echo "Feature branch avg: $(jq '.summary.avg_complexity' /tmp/feature-complexity.json)"

# Check if complexity increased
MAIN_AVG=$(jq '.summary.avg_complexity' /tmp/main-complexity.json)
FEATURE_AVG=$(jq '.summary.avg_complexity' /tmp/feature-complexity.json)

if (( $(echo "$FEATURE_AVG > $MAIN_AVG * 1.1" | bc -l) )); then
    echo "❌ Complexity increased by more than 10%!"
    exit 1
else
    echo "✅ Complexity is acceptable"
fi
```

## Filtering and Processing

### Find Files Above Threshold

```bash
# JSON output piped to jq
mccabre complexity src/ --json | \
  jq '.files[] | select(.cyclomatic.file_complexity > 10) | .path'
```

### Count Clone Groups

```bash
mccabre clones src/ --json | jq '.clones | length'
```

### Generate HTML

```bash
# Create HTML from JSON
mccabre analyze src/ --json | \
  jq -r '.files[] | "<tr><td>\(.path)</td><td>\(.cyclomatic.file_complexity)</td></tr>"' | \
  (echo "<html><table>" && cat && echo "</table></html>") > report.html
```

### Summary Statistics

```bash
# Extract just the summary
mccabre analyze src/ --json | jq '.summary'
```

### Generate and Save Configuration

Create a config file for your project:

```bash
# Save default config to current directory
mccabre dump-config -o mccabre.toml

# Generate config for a specific project structure
cd my-project/
mccabre dump-config -o .

# Copy config from one project to another
mccabre dump-config -c ../project-a/mccabre.toml -o ./mccabre.toml
```

## Tips and Tricks

### Syntax Highlighting

By default, code blocks in `analyze` and `clones` output are syntax highlighted. To disable:

```bash
# Disable syntax highlighting for cleaner output
mccabre analyze src/ --no-highlight

# Useful for piping to files or when colors aren't supported
mccabre clones src/ --no-highlight > report.txt
```

### Incremental Analysis

Analyze only changed files:

```bash
# Files changed in last commit
git diff --name-only HEAD~1 | grep '\.rs$' | xargs mccabre complexity
```

### Watch Mode

Continuously monitor (requires `watch` or `entr`):

```bash
# Using entr
ls src/**/*.rs | entr mccabre complexity src/
```

### Focus on New Code

Analyze only files in your feature branch:

```bash
git diff --name-only main...HEAD | grep '\.rs$' | xargs mccabre analyze
```

## Next Steps

- Read about [Cyclomatic Complexity](./cyclomatic-complexity.md)
- Learn about [Clone Detection](./clone-detection.md)
- Configure [thresholds](./configuration.md)
- Check the [CLI Reference](./cli-reference.md)
