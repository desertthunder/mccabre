use std::collections::BTreeMap;

pub fn compute_miss_ranges(lines: &BTreeMap<u32, u64>) -> Vec<(u32, u32)> {
    let mut miss_ranges = Vec::new();
    let mut current_range: Option<(u32, u32)> = None;

    for (&line, &count) in lines {
        if count == 0 {
            match current_range {
                None => current_range = Some((line, line)),
                Some((start, end)) => {
                    if line == end + 1 {
                        current_range = Some((start, line));
                    } else {
                        miss_ranges.push(current_range.unwrap());
                        current_range = Some((line, line));
                    }
                }
            }
        } else if let Some(range) = current_range {
            miss_ranges.push(range);
            current_range = None;
        }
    }

    if let Some(range) = current_range {
        miss_ranges.push(range);
    }

    miss_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_misses() {
        let mut lines = BTreeMap::new();
        lines.insert(1, 10);
        lines.insert(2, 5);
        lines.insert(3, 1);

        let ranges = compute_miss_ranges(&lines);
        assert!(ranges.is_empty());
    }

    #[test]
    fn test_single_miss() {
        let mut lines = BTreeMap::new();
        lines.insert(1, 10);
        lines.insert(2, 0);
        lines.insert(3, 5);

        let ranges = compute_miss_ranges(&lines);
        assert_eq!(ranges, vec![(2, 2)]);
    }

    #[test]
    fn test_consecutive_misses() {
        let mut lines = BTreeMap::new();
        lines.insert(1, 10);
        lines.insert(2, 0);
        lines.insert(3, 0);
        lines.insert(4, 0);
        lines.insert(5, 5);

        let ranges = compute_miss_ranges(&lines);
        assert_eq!(ranges, vec![(2, 4)]);
    }

    #[test]
    fn test_multiple_miss_ranges() {
        let mut lines = BTreeMap::new();
        lines.insert(1, 10);
        lines.insert(2, 0);
        lines.insert(3, 0);
        lines.insert(4, 5);
        lines.insert(5, 0);
        lines.insert(6, 10);

        let ranges = compute_miss_ranges(&lines);
        assert_eq!(ranges, vec![(2, 3), (5, 5)]);
    }

    #[test]
    fn test_all_misses() {
        let mut lines = BTreeMap::new();
        lines.insert(1, 0);
        lines.insert(2, 0);
        lines.insert(3, 0);

        let ranges = compute_miss_ranges(&lines);
        assert_eq!(ranges, vec![(1, 3)]);
    }

    #[test]
    fn test_empty_lines() {
        let lines = BTreeMap::new();
        let ranges = compute_miss_ranges(&lines);
        assert!(ranges.is_empty());
    }

    #[test]
    fn test_miss_at_start() {
        let mut lines = BTreeMap::new();
        lines.insert(1, 0);
        lines.insert(2, 0);
        lines.insert(3, 10);

        let ranges = compute_miss_ranges(&lines);
        assert_eq!(ranges, vec![(1, 2)]);
    }

    #[test]
    fn test_miss_at_end() {
        let mut lines = BTreeMap::new();
        lines.insert(1, 10);
        lines.insert(2, 5);
        lines.insert(3, 0);
        lines.insert(4, 0);

        let ranges = compute_miss_ranges(&lines);
        assert_eq!(ranges, vec![(3, 4)]);
    }
}
