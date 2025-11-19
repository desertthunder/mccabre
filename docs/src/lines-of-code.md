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

### Commands

Mccabre provides multiple ways to analyze LOC:

```bash
# Dedicated LOC analysis with ranking
mccabre loc src/

# Full analysis including complexity and clones
mccabre analyze src/

# Complexity analysis includes LOC
mccabre complexity src/
```

### The `loc` Command

The `loc` command provides focused LOC analysis with powerful ranking capabilities.

#### Basic Usage

```bash
# Rank files by logical LOC (default)
mccabre loc src/

# Rank files by physical LOC
mccabre loc src/ --rank-by physical

# Rank files by comments
mccabre loc src/ --rank-by comments

# Rank files by blank lines
mccabre loc src/ --rank-by blank
```

#### Directory Ranking

Group and rank files by directory:

```bash
# Rank directories by total LOC, files ranked within each
mccabre loc src/ --rank-dirs

# Rank directories by comments
mccabre loc src/ --rank-dirs --rank-by comments
```

### Sample Output

#### File Ranking

```text
LINES OF CODE ANALYSIS

SUMMARY
Total files analyzed:        12
Total physical LOC:          2246
Total logical LOC:           1360
Total comment lines:         135
Total blank lines:           751

FILES RANKED BY Logical LOC

1. crates/core/src/complexity/loc.rs (Logical LOC: 297)
   Physical: 403 | Logical: 297 | Comments: 37 | Blank: 69

2. crates/core/src/reporter.rs (Logical LOC: 212)
   Physical: 260 | Logical: 212 | Comments: 16 | Blank: 32
```

#### Directory Ranking

```text
DIRECTORIES RANKED BY Logical LOC

DIRECTORY: crates/core/src
  Total Physical:  1126 | Logical:  583 | Comments: 42 | Blank: 501

  Files:
    reporter.rs (Logical LOC: 212) - P: 260 | L: 212 | C: 16 | B: 32
    loader.rs (Logical LOC: 150) - P: 204 | L: 150 | C: 8 | B: 46
```

### JSON Output

```bash
# File ranking as JSON
mccabre loc src/ --json

# Directory ranking as JSON
mccabre loc src/ --rank-dirs --json
```

```json
{
  "files": [
    {
      "path": "src/main.rs",
      "metrics": {
        "physical": 120,
        "logical": 85,
        "comments": 25,
        "blank": 10
      }
    }
  ],
  "directories": null,
  "summary": {
    "total_files": 1,
    "total_physical": 120,
    "total_logical": 85,
    "total_comments": 25,
    "total_blank": 10
  }
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

## Advanced Use Cases

### Finding the Largest Files

```bash
# Top 10 largest files by logical LOC
mccabre loc src/ --rank-by logical | head -30
```

### Identifying Under-Commented Code

```bash
# Files ranked by comment count (ascending)
mccabre loc src/ --rank-by comments
```

### Directory Hotspots

```bash
# Find directories with the most code
mccabre loc . --rank-dirs --rank-by logical
```

### Tracking LOC Over Time

```bash
# Create baseline
mccabre loc src/ --json > baseline.json

# Later...
mccabre loc src/ --json > current.json

# Compare using jq
jq '.summary.total_logical' baseline.json
jq '.summary.total_logical' current.json
```

### CI Integration

Integrate with your CI to track:

- LOC growth per sprint
- LOC per feature
- Comment ratio trends
- Detect large file additions

## See Also

- [Cyclomatic Complexity](./cyclomatic-complexity.md)
- [Clone Detection](./clone-detection.md)
- [Configuration](./configuration.md)
