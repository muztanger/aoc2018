use aoc2018::read_input;
use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> (HashSet<char>, HashMap<char, Vec<char>>) {
    let mut dependencies: HashMap<char, Vec<char>> = HashMap::new();
    let mut all_steps: HashSet<char> = HashSet::new();

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() >= 10 {
            let prerequisite = words[1].chars().next().unwrap();
            let step = words[7].chars().next().unwrap();
            
            all_steps.insert(prerequisite);
            all_steps.insert(step);
            
            dependencies.entry(step).or_insert_with(Vec::new).push(prerequisite);
        }
    }

    (all_steps, dependencies)
}

fn part1(input: &str) -> String {
    let (mut all_steps, mut dependencies) = parse_input(input);
    let mut result = String::new();

    while !all_steps.is_empty() {
        // Find all steps with no remaining dependencies
        let mut available: Vec<char> = all_steps
            .iter()
            .filter(|&&step| {
                !dependencies.contains_key(&step) || dependencies[&step].is_empty()
            })
            .copied()
            .collect();

        // Sort alphabetically and take the first one
        available.sort();
        let next_step = available[0];

        // Add to result and remove from available steps
        result.push(next_step);
        all_steps.remove(&next_step);

        // Remove this step as a dependency from all other steps
        for deps in dependencies.values_mut() {
            deps.retain(|&c| c != next_step);
        }
    }

    result
}

fn part2(input: &str, num_workers: usize, base_time: u32) -> u32 {
    let (all_steps, mut dependencies) = parse_input(input);
    let mut completed: HashSet<char> = HashSet::new();
    let mut in_progress: HashMap<char, u32> = HashMap::new(); // step -> time remaining
    let mut time = 0;

    loop {
        // Process completed steps
        let finished: Vec<char> = in_progress
            .iter()
            .filter(|(_, &remaining)| remaining == 0)
            .map(|(&step, _)| step)
            .collect();

        for step in finished {
            in_progress.remove(&step);
            completed.insert(step);

            // Remove as dependency
            for deps in dependencies.values_mut() {
                deps.retain(|&c| c != step);
            }
        }

        // Check if we're done
        if completed.len() == all_steps.len() {
            break;
        }

        // Find available steps
        let mut available: Vec<char> = all_steps
            .iter()
            .filter(|&&step| {
                !completed.contains(&step)
                    && !in_progress.contains_key(&step)
                    && (!dependencies.contains_key(&step) || dependencies[&step].is_empty())
            })
            .copied()
            .collect();

        available.sort();

        // Assign work to available workers
        for step in available.iter().take(num_workers - in_progress.len()) {
            let step_time = base_time + (*step as u32 - 'A' as u32 + 1);
            in_progress.insert(*step, step_time);
        }

        // Advance time
        if !in_progress.is_empty() {
            let min_time = *in_progress.values().min().unwrap();
            for remaining in in_progress.values_mut() {
                *remaining -= min_time;
            }
            time += min_time;
        }
    }

    time
}

fn main() {
    let input = read_input(7);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input, 5, 60));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "CABDFE");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT, 2, 0), 15);
    }
}
