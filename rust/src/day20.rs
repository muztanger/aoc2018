use aoc2018::read_input;
use std::collections::HashMap;

fn parse_regex(regex: &str) -> HashMap<(i32, i32), i32> {
    let chars: Vec<char> = regex.chars().collect();
    let mut distances: HashMap<(i32, i32), i32> = HashMap::new();
    let mut stack: Vec<(i32, i32)> = Vec::new();
    let mut pos = (0, 0);
    
    distances.insert(pos, 0);
    
    for &ch in &chars {
        match ch {
            '^' | '$' => {},
            'N' | 'S' | 'E' | 'W' => {
                let next_pos = match ch {
                    'N' => (pos.0, pos.1 - 1),
                    'S' => (pos.0, pos.1 + 1),
                    'E' => (pos.0 + 1, pos.1),
                    'W' => (pos.0 - 1, pos.1),
                    _ => unreachable!(),
                };
                
                let curr_dist = *distances.get(&pos).unwrap_or(&0);
                let next_dist = distances.entry(next_pos).or_insert(i32::MAX);
                *next_dist = (*next_dist).min(curr_dist + 1);
                
                pos = next_pos;
            },
            '(' => {
                stack.push(pos);
            },
            ')' => {
                pos = stack.pop().unwrap();
            },
            '|' => {
                pos = *stack.last().unwrap();
            },
            _ => {},
        }
    }
    
    distances
}

fn part1(input: &str) -> i32 {
    let regex = input.trim();
    let distances = parse_regex(regex);
    
    *distances.values().max().unwrap_or(&0)
}

fn part2(input: &str) -> usize {
    let regex = input.trim();
    let distances = parse_regex(regex);
    
    distances.values().filter(|&&d| d >= 1000).count()
}

fn main() {
    let input = read_input(20);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("^WNE$"), 3);
        assert_eq!(part1("^ENWWW(NEEE|SSE(EE|N))$"), 10);
        assert_eq!(part1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"), 18);
        assert_eq!(part1("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"), 23);
        assert_eq!(part1("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"), 31);
    }
}
