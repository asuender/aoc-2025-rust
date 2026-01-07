# AGENTS.md - Agentic Coding Guidelines

This document provides guidelines for AI coding agents working in this repository.

## Project Overview

Rust implementation of Advent of Code 2025 solutions. Binary crate that accepts
command-line arguments to run specific day/part combinations.

- **Language**: Rust (Edition 2024)
- **Toolchain**: Rust 1.92.0+

## Project Structure

```
src/
  main.rs      # Entry point, CLI parsing, day dispatcher
  day01.rs     # Day 1 solutions (part01, part02 functions)
  ...
```

Each day module exports: `pub fn part01(input: &str)` and `pub fn part02(input: &str)`

## Build Commands

```bash
cargo check              # Check compilation without building
cargo build              # Build debug version
cargo build --release    # Build optimized release version
cargo run -- <day> <part> <input|example>   # Run (e.g., cargo run -- 1 1 input)
```

## Testing Commands

```bash
cargo test               # Run all tests
cargo test day01         # Run tests for specific module
cargo test test_name     # Run single test by name
cargo test -- --nocapture    # Run with output shown
cargo test --release     # Run in release mode
```

## Linting and Formatting

```bash
cargo fmt                # Format all code
cargo fmt -- --check     # Check formatting only
cargo clippy             # Run Clippy linter
cargo clippy -- -D warnings   # Deny all warnings (CI mode)
```

## Code Style Guidelines

### Imports

Group imports: 1) std::, 2) external crates, 3) local modules. Use nested imports:
```rust
use std::{env, fs, process::exit};
```

### Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Functions/Variables | snake_case | `part01`, `left_list` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_DAYS` |
| Types/Structs | PascalCase | `Config` |
| Modules | snake_case | `day01` |

### Function Signatures

```rust
// Day solution: takes &str, prints result
pub fn part01(input: &str) {
    let result = solve(input);
    println!("{result}");
}

// Helper: return Result when fallible
fn parse_numbers(line: &str) -> Result<Vec<i32>, ParseIntError> {
    line.split_whitespace().map(|s| s.parse()).collect()
}
```

### Error Handling

- Use `Result<T, Box<dyn Error>>` for functions that can fail multiple ways
- Prefer `?` operator for error propagation
- Use `eprintln!` for error output

```rust
let config = Config::build(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {err}");
    exit(1)
});

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    Ok(())
}
```

### Types

- Prefer `&str` over `String` for parameters when ownership not needed
- Use `Vec<T>` for dynamic arrays, `&[T]` for borrowed sequences
- Use iterators and method chaining over explicit loops

### Comments

- Add comments for complex algorithms or non-obvious logic
- Do not over-comment self-explanatory code
- Use `///` for public function documentation

### Common AoC Patterns

```rust
// Parse lines
let lines: Vec<&str> = input.lines().collect();

// Parse numbers
let nums: Vec<i32> = line.split_whitespace()
    .filter_map(|s| s.parse().ok()).collect();

// Grid parsing
let grid: Vec<Vec<char>> = input.lines()
    .map(|line| line.chars().collect()).collect();

// Count occurrences
use std::collections::HashMap;
let mut counts: HashMap<&str, usize> = HashMap::new();
for item in items {
    *counts.entry(item).or_insert(0) += 1;
}
```

## Adding a New Day

1. Create `src/dayXX.rs` with `part01` and `part02` functions
2. Add `mod dayXX;` to `src/main.rs`
3. Add match arms in the `run` function dispatcher
4. Run with `cargo run -- XX 1 input`

## Pre-commit Checklist

1. `cargo fmt` - Format code
2. `cargo clippy` - Fix warnings
3. `cargo test` - Run tests
4. `cargo build` - Verify compilation

## Performance Tips

- Use `cargo run --release` for optimized execution
- Consider `rustc-hash` for faster HashMaps
- Use `itertools` for additional iterator methods

## Feedback workflow

This is primarily a learning project. The user has prior experience in Python and Typescript and wants to extend his knowledge by learning Rust. The user might therefore ask you to give feedback on his work (which is expected not to be perfect) on a regular basis. In such cases, follow this workflow:

- Gather the context about the code on which you have to give feedback
- If needed, fetch and read the task description from the AoC website
- Assess if the present solution can be refactored using features that Rustaceans would use
  - if a better solution is reasonable given the user's experience with Rust so far:
    - propose and explain it to him in detail
  - otherwise: if the refactoring involves advanced features
    - keep it as-is and just mention how it can be solved differently

