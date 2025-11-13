# Quick Start

This guide will get you analyzing code in minutes.

## Basic Usage

### Analyze Everything

Run a full analysis on a directory:

```bash
mccabre analyze ./src
```

This will show:

- Cyclomatic complexity per file
- Lines of code metrics
- Detected code clones

### Complexity Only

To check only complexity metrics:

```bash
mccabre complexity ./src
```

### Clone Detection Only

To find duplicated code:

```bash
mccabre clones ./src
```

## Understanding the Output

### Terminal Output

Mccabre uses colors to highlight issues:

- **Green**: Low complexity (1-10)
- **Yellow**: Moderate complexity (11-20)
- **Red**: High complexity (21+)

Example output:

```text
================================================================================
MCCABRE CODE ANALYSIS REPORT
================================================================================

SUMMARY
--------------------------------------------------------------------------------
Total files analyzed:        5
Total physical LOC:          450
Total logical LOC:           320
Average complexity:          8.50
Maximum complexity:          18
High complexity files:       2
Clone groups detected:       3

FILE METRICS
--------------------------------------------------------------------------------
FILE: src/utils.rs
    Cyclomatic Complexity:   5 (low)
    Physical LOC:            45
    Logical LOC:             32
```

### JSON

```bash
mccabre analyze ./src --json > report.json
```

## Next Steps

- Learn about [Cyclomatic Complexity](./cyclomatic-complexity.md)
- Understand [Clone Detection](./clone-detection.md)
- Configure [thresholds](./configuration.md)
- See more [examples](./examples.md)
