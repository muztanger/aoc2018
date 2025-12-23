# Advent of Code 2018 - Rust Solutions

Solutions to [Advent of Code 2018](https://adventofcode.com/2018) problems in Rust.

## Structure

- `src/` - Contains solution files for each day (day01.rs, day02.rs, etc.)
- `src/lib.rs` - Common utilities and helper functions
- `inputs/` - Input files for each day (day01.txt, day02.txt, etc.)

## Running Solutions

To run a specific day's solution:

```bash
cargo run --bin day01
```

To run with release optimizations (recommended for days with heavy computation):

```bash
cargo run --release --bin day01
```

## Testing

Run tests for all solutions:

```bash
cargo test
```

Run tests for a specific day:

```bash
cargo test --bin day01
```

## Adding a New Day

Each day's solution should follow this template structure:

```rust
use aoc2018::read_input;

fn main() {
    let input = read_input(DAY_NUMBER);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> ReturnType {
    // Solution implementation
}

fn part2(input: &str) -> ReturnType {
    // Solution implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // Test cases from problem description
    }

    #[test]
    fn test_part2() {
        // Test cases from problem description
    }
}
```

## Input Files

Place your input files in the `inputs/` directory with the naming convention `dayXX.txt` (e.g., `day01.txt`, `day02.txt`).
