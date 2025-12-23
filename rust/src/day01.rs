use aoc2018::read_input;
use std::collections::HashSet;

fn part1(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| line.trim().parse::<i32>().ok())
        .sum()
}

fn part2(input: &str) -> i32 {
    let changes: Vec<i32> = input
        .lines()
        .filter_map(|line| line.trim().parse::<i32>().ok())
        .collect();

    let mut frequency = 0;
    let mut seen = HashSet::new();
    seen.insert(0);

    for change in changes.iter().cycle() {
        frequency += change;
        if !seen.insert(frequency) {
            return frequency;
        }
    }

    unreachable!()
}

fn main() {
    let input = read_input(1);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("+1\n-2\n+3\n+1"), 3);
        assert_eq!(part1("+1\n+1\n+1"), 3);
        assert_eq!(part1("+1\n+1\n-2"), 0);
        assert_eq!(part1("-1\n-2\n-3"), -6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("+1\n-2\n+3\n+1"), 2);
        assert_eq!(part2("+1\n-1"), 0);
        assert_eq!(part2("+3\n+3\n+4\n-2\n-4"), 10);
        assert_eq!(part2("-6\n+3\n+8\n+5\n-6"), 5);
        assert_eq!(part2("+7\n+7\n-2\n-7\n-4"), 14);
    }
}
