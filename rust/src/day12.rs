use aoc2018::read_input;
use std::collections::HashSet;

type Pots = HashSet<i64>;

fn parse_input(input: &str) -> (Pots, HashSet<[u8; 5]>) {
    let mut lines = input.lines();

    let initial_line = lines.next().unwrap();
    let initial_state = initial_line
        .strip_prefix("initial state: ")
        .unwrap()
        .trim();

    let mut plants: Pots = HashSet::new();
    for (i, ch) in initial_state.bytes().enumerate() {
        if ch == b'#' {
            plants.insert(i as i64);
        }
    }

    // Skip blank line
    lines.next();

    let mut rules = HashSet::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let (pat, res) = line.split_once(" => ").unwrap();
        if res.as_bytes()[0] == b'#' {
            let mut arr = [b'.'; 5];
            for (i, c) in pat.bytes().enumerate() {
                arr[i] = c;
            }
            rules.insert(arr);
        }
    }

    (plants, rules)
}

fn next_generation(current: &Pots, rules: &HashSet<[u8; 5]>) -> Pots {
    if current.is_empty() {
        return Pots::new();
    }

    let min = current.iter().min().cloned().unwrap();
    let max = current.iter().max().cloned().unwrap();

    let mut next = Pots::new();

    // Only positions within two of existing plants can change
    for i in (min - 2)..=(max + 2) {
        let mut pat = [b'.'; 5];
        for (idx, offset) in (-2..=2).enumerate() {
            if current.contains(&(i + offset)) {
                pat[idx] = b'#';
            }
        }

        if rules.contains(&pat) {
            next.insert(i);
        }
    }

    next
}

fn sum_of_plants(plants: &Pots) -> i64 {
    plants.iter().sum()
}

fn part1(input: &str) -> i64 {
    let (mut plants, rules) = parse_input(input);

    for _ in 0..20 {
        plants = next_generation(&plants, &rules);
    }

    sum_of_plants(&plants)
}

fn part2(input: &str) -> i64 {
    let target: i64 = 50_000_000_000;
    let (mut plants, rules) = parse_input(input);

    let mut last_sum = sum_of_plants(&plants);
    let mut last_delta = 0;
    let mut stable_steps = 0;

    let mut generation: i64 = 0;

    while generation < target {
        generation += 1;
        plants = next_generation(&plants, &rules);
        let current_sum = sum_of_plants(&plants);
        let delta = current_sum - last_sum;

        if delta == last_delta {
            stable_steps += 1;
        } else {
            stable_steps = 0;
            last_delta = delta;
        }

        // Heuristic: once the delta has been stable for 200 generations,
        // assume linear growth and fast-forward the remaining steps.
        if stable_steps >= 200 {
            let remaining = target - generation;
            return current_sum + remaining * delta;
        }

        last_sum = current_sum;
    }

    last_sum
}

fn main() {
    let input = read_input(12);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 325);
    }

    #[test]
    fn test_part2_convergence() {
        // The sample does not provide a part 2 answer; ensure the code executes.
        let _ = part2(TEST_INPUT);
    }
}
