# Clone Detection

## What is Code Cloning?

Code clones are similar or identical code fragments that appear in multiple places. They indicate duplication and potential refactoring opportunities.

## How Mccabre Detects Clones

Mccabre uses **Rabin-Karp rolling hash**, a fast string matching algorithm adapted for token sequences.

### Algorithm Overview

1. **Tokenization**: Convert source code to tokens
2. **Windowing**: Slide a window of N tokens across the sequence
3. **Hashing**: Compute a rolling hash for each window
4. **Matching**: Identify windows with identical hashes
5. **Reporting**: Group matches into clone groups

### Why This Approach?

**Advantages:**

- **Fast**: O(n) time complexity
- **Language-agnostic**: Works on tokens, not syntax trees
- **Tunable**: Adjust window size to find smaller or larger clones

**Trade-offs:**

- Finds exact token matches only
- Doesn't detect semantic equivalence
- May miss clones with renamed variables

## Using Clone Detection

### Basic Usage

```bash
mccabre clones src/
```

### Adjust Sensitivity

The `--min-tokens` flag controls the minimum clone size:

```bash
# Find larger clones (more strict)
mccabre clones src/ --min-tokens 50

# Find smaller clones (more sensitive)
mccabre clones src/ --min-tokens 15
```

### Sample Output

```text
DETECTED CLONES
--------------------------------------------------------------------------------
Clone Group #1 (length: 32 tokens, 3 occurrences)
  - src/user.go:15-28
  - src/product.go:42-55
  - src/order.go:88-101

Clone Group #2 (length: 45 tokens, 2 occurrences)
  - src/validators.rs:120-145
  - src/sanitizers.rs:67-92
```

## Interpreting Results

### Clone Group Fields

- **ID**: Unique identifier for the clone group
- **Length**: Number of tokens in the duplicated sequence
- **Occurrences**: How many times this clone appears
- **Locations**: File paths and line ranges

### Significance

| Tokens | Significance | Action |
|--------|-------------|--------|
| 15-25  | Minor duplication | Consider refactoring if repeated 3+ times |
| 26-50  | Moderate duplication | Should refactor |
| 50+    | Major duplication | Urgent refactoring needed |

## Refactoring Clones

### Example: Extract Function

**Before:**

```go
// In file1.go
func processUser(input string) string {
    trimmed := strings.TrimSpace(input)
    if len(trimmed) == 0 {
        return ""
    }
    lower := strings.ToLower(trimmed)
    return lower
}

// In file2.go
func processProduct(name string) string {
    trimmed := strings.TrimSpace(name)
    if len(trimmed) == 0 {
        return ""
    }
    lower := strings.ToLower(trimmed)
    return lower
}
```

**After:**

```go
// In utils.go
func sanitizeString(input string) string {
    trimmed := strings.TrimSpace(input)
    if len(trimmed) == 0 {
        return ""
    }
    return strings.ToLower(trimmed)
}

// In file1.go
func processUser(input string) string {
    return sanitizeString(input)
}

// In file2.go
func processProduct(name string) string {
    return sanitizeString(name)
}
```

### Example: Extract Class/Module

**Before:** Multiple files with similar validation logic

**After:** Single `validation` module imported by all files

## Types of Clones

### Type 1: Exact Clones

Identical code except for whitespace and comments.

```javascript
// Clone 1
function calc(a, b) {
    return a + b;
}

// Clone 2
function calc(a, b) {
    return a + b;
}
```

✅ **Mccabre detects these**

### Type 2: Renamed Clones

Identical except for variable/function names.

```javascript
// Clone 1
function add(x, y) {
    return x + y;
}

// Clone 2
function sum(a, b) {
    return a + b;
}
```

❌ **Mccabre does NOT detect these** (yet)

### Type 3: Near-Miss Clones

Similar structure with minor modifications.

```javascript
// Clone 1
function validate(user) {
    if (!user.email) return false;
    if (!user.name) return false;
    return true;
}

// Clone 2
function validate(product) {
    if (!product.id) return false;
    if (!product.price) return false;
    if (!product.name) return false;
    return true;
}
```

❌ **Mccabre does NOT detect these**

### Type 4: Semantic Clones

Different syntax, same behavior.

```javascript
// Clone 1
const sum = arr.reduce((a, b) => a + b, 0);

// Clone 2
let sum = 0;
for (let num of arr) {
    sum += num;
}
```

❌ **Mccabre does NOT detect these**

## Configuration

### Via Command Line

```bash
mccabre clones . --min-tokens 30
```

### Via Config File

Create `mccabre.toml`:

```toml
[clones]
enabled = true
min_tokens = 30
```

## JSON Output

```bash
mccabre clones src/ --json
```

```json
{
  "clones": [
    {
      "id": 1,
      "length": 32,
      "locations": [
        {
          "file": "src/user.go",
          "start_line": 15,
          "end_line": 28
        },
        {
          "file": "src/product.go",
          "start_line": 42,
          "end_line": 55
        }
      ]
    }
  ]
}
```

## References

- [Rabin-Karp Algorithm](https://en.wikipedia.org/wiki/Rabin%E2%80%93Karp_algorithm)
- [Code Clone Research](https://www.sei.cmu.edu/library/code-similarity-detection-using-syntax-agnostic-locality-sensitive-hashing/)

## See Also

- [Cyclomatic Complexity](./cyclomatic-complexity.md)
- [CLI Reference](./cli-reference.md)
- [Examples](./examples.md)
