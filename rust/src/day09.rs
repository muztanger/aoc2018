use aoc2018::read_input;
use std::collections::VecDeque;

fn parse_input(input: &str) -> (usize, usize) {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let players = parts[0].parse().unwrap();
    let last_marble = parts[6].parse().unwrap();
    (players, last_marble)
}

fn play_game(players: usize, last_marble: usize) -> usize {
    let mut scores = vec![0; players];
    let mut circle: VecDeque<usize> = VecDeque::new();
    circle.push_back(0);
    
    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            // Special scoring rule
            let player = (marble - 1) % players;
            scores[player] += marble;
            
            // Rotate 7 positions counter-clockwise
            for _ in 0..7 {
                let back = circle.pop_back().unwrap();
                circle.push_front(back);
            }
            
            // Remove and score the current marble
            let removed = circle.pop_front().unwrap();
            scores[player] += removed;
            
            // Next marble becomes current (it's already at front)
        } else {
            // Normal placement: rotate 2 clockwise, then insert
            for _ in 0..2 {
                let front = circle.pop_front().unwrap();
                circle.push_back(front);
            }
            circle.push_front(marble);
        }
    }
    
    *scores.iter().max().unwrap()
}

fn part1(input: &str) -> usize {
    let (players, last_marble) = parse_input(input);
    play_game(players, last_marble)
}

fn part2(input: &str) -> usize {
    let (players, last_marble) = parse_input(input);
    play_game(players, last_marble * 100)
}

fn main() {
    let input = read_input(9);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(play_game(9, 25), 32);
        assert_eq!(play_game(10, 1618), 8317);
        assert_eq!(play_game(13, 7999), 146373);
        assert_eq!(play_game(17, 1104), 2764);
        assert_eq!(play_game(21, 6111), 54718);
        assert_eq!(play_game(30, 5807), 37305);
    }
}
