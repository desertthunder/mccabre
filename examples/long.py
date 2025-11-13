#!/usr/bin/env python3
"""
Example Python file with many lines of code
Demonstrates LOC counting with comments and blank lines
"""

import sys
import os
from typing import List, Dict, Optional


class DataProcessor:
    """Process data with various transformations"""

    def __init__(self, config: Dict):
        self.config = config
        self.data = []
        self.results = {}

    def load_data(self, filepath: str) -> bool:
        """Load data from a file"""
        try:
            with open(filepath, 'r') as f:
                self.data = [line.strip() for line in f]
            return True
        except FileNotFoundError:
            print(f"File not found: {filepath}")
            return False

    def process(self) -> List[str]:
        """Process the loaded data"""
        results = []

        for item in self.data:
            # Skip empty lines
            if not item:
                continue

            # Transform the item
            transformed = self._transform(item)

            # Validate
            if self._validate(transformed):
                results.append(transformed)

        return results

    def _transform(self, item: str) -> str:
        """Transform a single item"""
        # Convert to lowercase
        item = item.lower()

        # Remove special characters
        item = ''.join(c for c in item if c.isalnum() or c.isspace())

        # Trim whitespace
        item = item.strip()

        return item

    def _validate(self, item: str) -> bool:
        """Validate an item"""
        if len(item) < 3:
            return False

        if not any(c.isalpha() for c in item):
            return False

        return True

    def save_results(self, output_path: str) -> None:
        """Save processed results"""
        with open(output_path, 'w') as f:
            for item in self.results:
                f.write(f"{item}\n")


def main():
    """Main entry point"""
    if len(sys.argv) < 2:
        print("Usage: python long.py <input_file>")
        sys.exit(1)

    processor = DataProcessor({"strict": True})
    processor.load_data(sys.argv[1])
    results = processor.process()

    print(f"Processed {len(results)} items")


if __name__ == "__main__":
    main()

# Physical LOC: ~95
# Logical LOC: ~60
# Comment lines: ~15
# Blank lines: ~20
