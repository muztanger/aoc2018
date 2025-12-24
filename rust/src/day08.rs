use aoc2018::read_input;

struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>,
}

impl Node {
    fn parse(numbers: &[i32], index: &mut usize) -> Self {
        let num_children = numbers[*index];
        *index += 1;
        let num_metadata = numbers[*index];
        *index += 1;

        let mut children = Vec::new();
        for _ in 0..num_children {
            children.push(Node::parse(numbers, index));
        }

        let mut metadata = Vec::new();
        for _ in 0..num_metadata {
            metadata.push(numbers[*index]);
            *index += 1;
        }

        Node { children, metadata }
    }

    fn sum_metadata(&self) -> i32 {
        let mut sum = self.metadata.iter().sum::<i32>();
        for child in &self.children {
            sum += child.sum_metadata();
        }
        sum
    }

    fn value(&self) -> i32 {
        if self.children.is_empty() {
            // If no children, value is sum of metadata
            self.metadata.iter().sum()
        } else {
            // If has children, metadata entries are 1-based indices into children
            let mut sum = 0;
            for &index in &self.metadata {
                if index > 0 && (index as usize) <= self.children.len() {
                    sum += self.children[(index - 1) as usize].value();
                }
            }
            sum
        }
    }
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1(input: &str) -> i32 {
    let numbers = parse_input(input);
    let mut index = 0;
    let root = Node::parse(&numbers, &mut index);
    root.sum_metadata()
}

fn part2(input: &str) -> i32 {
    let numbers = parse_input(input);
    let mut index = 0;
    let root = Node::parse(&numbers, &mut index);
    root.value()
}

fn main() {
    let input = read_input(8);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 138);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 66);
    }
}
