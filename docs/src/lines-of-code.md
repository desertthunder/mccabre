# Lines of Code (LOC)

## Overview

Lines of Code (LOC) is a fundamental software metric that measures the size of a codebase. Mccabre provides several LOC variants to give you a complete picture.

## Metrics Provided

### Physical LOC

Total number of lines in the file, including everything.

```rust
fn hello() {        // Line 1


    println!("Hi"); // Line 4
}                   // Line 5
// Physical LOC = 5
```

### Logical LOC

Non-blank, non-comment lines that contain actual code.

```rust
fn hello() {        // Logical
    // This is a comment (not counted)

    println!("Hi"); // Logical
}                   // Logical
// Logical LOC = 3
```

### Comment Lines

Lines that contain comments (single-line or multi-line).

```rust
// This is counted        // Comment line
/* This too */           // Comment line
let x = 5; // inline     // Code line (code takes precedence)
```

### Blank Lines

Lines that contain only whitespace.

## Why LOC Matters

### Productivity Tracking

- Monitor codebase growth over time
- Estimate project size
- Compare different implementations

### Maintenance Effort

Larger codebases generally require:

- More time to understand
- More effort to maintain
- More potential for bugs

### Code Density

Compare logical vs physical LOC:

- High ratio (logical/physical): Dense code, few comments
- Low ratio: Well-commented, more likely to be readable code

## Using LOC with Mccabre

### Basic Usage

```bash
# Analyze LOC for a directory
mccabre analyze src/

# Complexity command also includes LOC
mccabre complexity src/
```

### Sample Output

```text
FILE: src/main.rs
    Cyclomatic Complexity:   5
    Physical LOC:            120
    Logical LOC:             85
    Comment lines:           25
    Blank lines:             10
```

### JSON

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
      }
    }
  ]
}
```

## Interpreting Results

### Healthy Ratios

**Comment Ratio**: `comments / logical`

- 0.1-0.3 (10-30%): Generally good
- <0.05: Likely under-commented
- >0.5: Possibly over-commented or tutorial code

**Code Density**: `logical / physical`

- 0.6-0.8: Good balance
- <0.5: Many blank lines/comments (verbose)
- >0.9: Very dense (potentially hard to read)

## LOC Limitations

### Not a Quality Metric

More LOC doesn't mean:

- ✗ Better code
- ✗ More features
- ✗ More value

### Context Matters

Compare LOC only within similar contexts:

- Same language
- Same problem domain
- Same team/style

### Generated Code

LOC counts everything, including:

- Auto-generated code
- Vendored dependencies
- Build artifacts

Use `.gitignore` to exclude these (Mccabre respects gitignore).

## Tracking LOC Over Time

### Baseline

```bash
# Create baseline
mccabre analyze src/ --json > baseline.json
```

### Compare

```bash
# Later...
mccabre analyze src/ --json > current.json

# Compare (using jq)
jq '.summary.total_logical_loc' baseline.json
jq '.summary.total_logical_loc' current.json
```

### Visualize Growth

Integrate with your CI to track:

- LOC growth per sprint
- LOC per feature
- Comment ratio trends

## See Also

- [Cyclomatic Complexity](./cyclomatic-complexity.md)
- [Clone Detection](./clone-detection.md)
- [Configuration](./configuration.md)
