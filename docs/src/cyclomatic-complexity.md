# Cyclomatic Complexity

## What is Cyclomatic Complexity?

Cyclomatic Complexity (CC), introduced by Thomas McCabe in 1976, measures the number of independent paths through a program's source code. It provides a quantitative measure of code complexity.

## How It Works

Mccabre uses a simplified formula:

`CC = (number of decision points) + 1`

### Decision Points

A decision point is any control flow statement that creates a branch:

- `if`, `else if`
- `while`, `for`, `loop`
- `switch`, `match`, `case`
- `catch`
- Logical operators: `&&`, `||`
- Ternary operator: `?`

### Example

```javascript
function checkUser(user) {  // CC starts at 1
    if (!user) {             // +1 = 2
        return false;
    }

    if (user.age > 18 && user.verified) {  // +1 (if) +1 (&&) = 4
        return true;
    }

    return false;
}
// Total CC = 4
```

## Interpretation

| CC Range | Risk Level | Recommendation |
|----------|-----------|----------------|
| 1-10 | Low | Simple, easy to test |
| 11-20 | Moderate | Consider refactoring |
| 21-50 | High | Should refactor |
| 50+ | Very High | Urgent refactoring needed |

## Why It Matters

### Testing Complexity

Higher CC means:

- More test cases needed for full coverage
- Higher chance of bugs
- Harder to understand and maintain

A function with CC=10 requires at least 10 test cases to cover all paths.

### Maintenance Burden

Complex functions are:

- Harder to modify without introducing bugs
- More difficult for new developers to understand
- More prone to subtle edge cases

## Reducing Complexity

### Extract Methods

**Before (CC=8):**

```javascript
function processOrder(order) {
    if (!order.id) throw new Error("No ID");
    if (order.status === "cancelled") return null;
    if (order.items.length === 0) throw new Error("No items");

    let total = 0;
    for (let item of order.items) {
        if (item.price && item.quantity) {
            total += item.price * item.quantity;
        }
    }

    return total;
}
```

**After (CC=3 + 3 = 6 total):**

```javascript
function processOrder(order) {  // CC=3
    validateOrder(order);
    return calculateTotal(order);
}

function validateOrder(order) {  // CC=3
    if (!order.id) throw new Error("No ID");
    if (order.status === "cancelled") return null;
    if (order.items.length === 0) throw new Error("No items");
}

function calculateTotal(order) {  // CC=2
    let total = 0;
    for (let item of order.items) {
        if (item.price && item.quantity) {
            total += item.price * item.quantity;
        }
    }
    return total;
}
```

### Use Early Returns

**Before:**

```rust
fn check(x: i32) -> bool {  // CC=3
    let mut result = false;
    if x > 0 {
        if x < 100 {
            result = true;
        }
    }
    result
}
```

**After:**

```rust
fn check(x: i32) -> bool {  // CC=3 (same but cleaner)
    if x <= 0 { return false; }
    if x >= 100 { return false; }
    true
}
```

### Replace Complex Conditions

**Before:**

```javascript
if ((user.role === "admin" || user.role === "moderator") &&
    user.active && !user.suspended) {  // CC contribution: 4
    // ...
}
```

**After:**

```javascript
function canModerate(user) {  // CC=3
    const isModerator = user.role === "admin" || user.role === "moderator";
    return isModerator && user.active && !user.suspended;
}

if (canModerate(user)) {  // CC=1
    // ...
}
```

## Using with Mccabre

### Check Specific Files

```bash
mccabre complexity src/complex.js
```

### Set Custom Threshold

```bash
mccabre complexity . --threshold 15
```

### JSON Output for CI

```bash
mccabre complexity . --json | jq '.files[] | select(.cyclomatic.file_complexity > 20)'
```

## References

- [McCabe (1976): "A Complexity Measure"](https://www.literateprogramming.com/mccabe.pdf)
- [NIST Special Publication 500-235](https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication500-235.pdf)

## See Also

- [Lines of Code](./lines-of-code.md)
- [Clone Detection](./clone-detection.md)
- [CLI Reference](./cli-reference.md)
