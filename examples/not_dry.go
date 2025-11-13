package main

import (
	"fmt"
	"strings"
)

// Example demonstrating code duplication (clones)
// Multiple functions have similar logic

func processUserInput(input string) string {
	// Trim and validate input
	trimmed := strings.TrimSpace(input)
	if len(trimmed) == 0 {
		return ""
	}

	// Convert to lowercase
	lower := strings.ToLower(trimmed)

	// Remove special characters
	cleaned := ""
	for _, char := range lower {
		if (char >= 'a' && char <= 'z') || (char >= '0' && char <= '9') || char == ' ' {
			cleaned += string(char)
		}
	}

	return cleaned
}

func processProductName(name string) string {
	// Trim and validate input
	trimmed := strings.TrimSpace(name)
	if len(trimmed) == 0 {
		return ""
	}

	// Convert to lowercase
	lower := strings.ToLower(trimmed)

	// Remove special characters
	cleaned := ""
	for _, char := range lower {
		if (char >= 'a' && char <= 'z') || (char >= '0' && char <= '9') || char == ' ' {
			cleaned += string(char)
		}
	}

	return cleaned
}

func sanitizeFileName(filename string) string {
	// Trim and validate input
	trimmed := strings.TrimSpace(filename)
	if len(trimmed) == 0 {
		return ""
	}

	// Convert to lowercase
	lower := strings.ToLower(trimmed)

	// Remove special characters
	cleaned := ""
	for _, char := range lower {
		if (char >= 'a' && char <= 'z') || (char >= '0' && char <= '9') || char == ' ' {
			cleaned += string(char)
		}
	}

	return cleaned
}

func main() {
	user := processUserInput("  Hello World!  ")
	product := processProductName("  Test Product 123  ")
	file := sanitizeFileName("  my_file.txt  ")

	fmt.Println(user)
	fmt.Println(product)
	fmt.Println(file)
}

// This file has obvious code duplication across the three functions
// Clone detection should identify the repeated token sequences
