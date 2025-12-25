use aoc2018::read_input;
use std::collections::{VecDeque, HashMap, HashSet};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum UnitType {
    Elf,
    Goblin,
}

#[derive(Clone, Debug)]
struct Unit {
    pos: (usize, usize),
    unit_type: UnitType,
    hp: i32,
    attack: i32,
}

#[derive(Clone)]
struct State {
    grid: Vec<Vec<char>>,
    units: Vec<Unit>,
}

impl State {
    fn from_input(input: &str, elf_attack: i32) -> Self {
        let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let mut units = Vec::new();

        for (y, row) in grid.iter_mut().enumerate() {
            for (x, ch) in row.iter_mut().enumerate() {
                let unit_type = match *ch {
                    'E' => Some(UnitType::Elf),
                    'G' => Some(UnitType::Goblin),
                    _ => None,
                };
                if let Some(ut) = unit_type {
                    let attack = match ut {
                        UnitType::Elf => elf_attack,
                        UnitType::Goblin => 3,
                    };
                    units.push(Unit {
                        pos: (x, y),
                        unit_type: ut,
                        hp: 200,
                        attack,
                    });
                    *ch = '.';
                }
            }
        }

        State { grid, units }
    }

    fn is_occupied(&self, pos: (usize, usize)) -> bool {
        self.units.iter().any(|u| u.hp > 0 && u.pos == pos)
    }

    fn is_open(&self, pos: (usize, usize)) -> bool {
        if pos.1 >= self.grid.len() || pos.0 >= self.grid[pos.1].len() {
            return false;
        }
        self.grid[pos.1][pos.0] == '.' && !self.is_occupied(pos)
    }

    fn neighbors(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        // Reading order: up, left, right, down
        if pos.1 > 0 {
            result.push((pos.0, pos.1 - 1));
        }
        if pos.0 > 0 {
            result.push((pos.0 - 1, pos.1));
        }
        if pos.0 + 1 < self.grid[0].len() {
            result.push((pos.0 + 1, pos.1));
        }
        if pos.1 + 1 < self.grid.len() {
            result.push((pos.0, pos.1 + 1));
        }
        result
    }

    fn find_move(&self, from: (usize, usize), unit_type: UnitType) -> Option<(usize, usize)> {
        let enemy_type = match unit_type {
            UnitType::Elf => UnitType::Goblin,
            UnitType::Goblin => UnitType::Elf,
        };
        
        // Find all target squares (empty squares adjacent to enemies)
        let mut targets = HashSet::new();
        for unit in &self.units {
            if unit.hp > 0 && unit.unit_type == enemy_type {
                for neighbor in self.neighbors(unit.pos) {
                    if self.is_open(neighbor) {
                        targets.insert(neighbor);
                    }
                }
            }
        }

        if targets.is_empty() {
            return None;
        }

        // BFS from starting position to find nearest target
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut distances = HashMap::new();
        
        queue.push_back(from);
        visited.insert(from);
        distances.insert(from, 0);

        let mut nearest_targets = Vec::new();
        let mut nearest_dist = None;

        while let Some(pos) = queue.pop_front() {
            let dist = distances[&pos];

            // If we've already found targets and this is farther, stop
            if let Some(nd) = nearest_dist {
                if dist > nd {
                    break;
                }
            }

            // Check if this is a target
            if targets.contains(&pos) {
                if nearest_dist.is_none() {
                    nearest_dist = Some(dist);
                }
                nearest_targets.push(pos);
                continue; // Don't explore past a target
            }

            // Explore neighbors in reading order
            for neighbor in self.neighbors(pos) {
                if !visited.contains(&neighbor) && self.is_open(neighbor) {
                    visited.insert(neighbor);
                    distances.insert(neighbor, dist + 1);
                    queue.push_back(neighbor);
                }
            }
        }

        if nearest_targets.is_empty() {
            return None;
        }

        // Choose target in reading order
        nearest_targets.sort_by_key(|&(x, y)| (y, x));
        let chosen_target = nearest_targets[0];

        // Now BFS from all neighbors of `from` to find which one gets to target first
        // and in case of tie, pick the one in reading order
        let neighbors = self.neighbors(from);
        let mut best_neighbor = None;
        let mut best_dist = None;

        for neighbor in neighbors {
            if !self.is_open(neighbor) {
                continue;
            }

            // BFS from this neighbor to the target
            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();
            let mut distances = HashMap::new();
            
            queue.push_back(neighbor);
            visited.insert(neighbor);
            distances.insert(neighbor, 0);

            while let Some(pos) = queue.pop_front() {
                if pos == chosen_target {
                    let dist = distances[&pos];
                    if best_dist.is_none() || dist < best_dist.unwrap() || 
                       (dist == best_dist.unwrap() && (neighbor.1, neighbor.0) < (best_neighbor.unwrap().1, best_neighbor.unwrap().0)) {
                        best_dist = Some(dist);
                        best_neighbor = Some(neighbor);
                    }
                    break;
                }

                let d = distances[&pos];
                for next in self.neighbors(pos) {
                    if !visited.contains(&next) && self.is_open(next) {
                        visited.insert(next);
                        distances.insert(next, d + 1);
                        queue.push_back(next);
                    }
                }
            }
        }

        best_neighbor
    }

    fn simulate_round(&mut self) -> bool {
        // Sort units by reading order
        let mut turn_order: Vec<usize> = (0..self.units.len()).collect();
        turn_order.sort_by_key(|&i| {
            let pos = self.units[i].pos;
            (pos.1, pos.0)
        });

        for &unit_idx in &turn_order {
            // Skip dead units
            if self.units[unit_idx].hp <= 0 {
                continue;
            }

            let unit_type = self.units[unit_idx].unit_type;
            
            // Check if there are any enemies left
            let has_enemies = self.units.iter().any(|u| u.hp > 0 && u.unit_type != unit_type);
            if !has_enemies {
                return false; // Combat ends
            }

            // Check if already adjacent to an enemy
            let pos = self.units[unit_idx].pos;
            let adjacent_enemies: Vec<usize> = self.units.iter().enumerate()
                .filter(|(_, u)| u.hp > 0 && u.unit_type != unit_type)
                .filter(|(_, u)| {
                    let (ux, uy) = u.pos;
                    let (px, py) = pos;
                    (ux == px && (uy.abs_diff(py) == 1)) ||
                    (uy == py && (ux.abs_diff(px) == 1))
                })
                .map(|(i, _)| i)
                .collect();

            // Move if not adjacent to enemy
            if adjacent_enemies.is_empty() {
                if let Some(next_pos) = self.find_move(pos, unit_type) {
                    self.units[unit_idx].pos = next_pos;
                }
            }

            // Attack (after potentially moving)
            let pos = self.units[unit_idx].pos;
            let adjacent_enemies: Vec<usize> = self.units.iter().enumerate()
                .filter(|(_, u)| u.hp > 0 && u.unit_type != unit_type)
                .filter(|(_, u)| {
                    let (ux, uy) = u.pos;
                    let (px, py) = pos;
                    (ux == px && (uy.abs_diff(py) == 1)) ||
                    (uy == py && (ux.abs_diff(px) == 1))
                })
                .map(|(i, _)| i)
                .collect();

            if !adjacent_enemies.is_empty() {
                // Pick target with lowest HP, ties broken by reading order
                let target_idx = adjacent_enemies.iter()
                    .min_by_key(|&&i| {
                        let u = &self.units[i];
                        (u.hp, u.pos.1, u.pos.0)
                    })
                    .copied()
                    .unwrap();
                
                let attack_power = self.units[unit_idx].attack;
                self.units[target_idx].hp -= attack_power;
            }
        }

        // Remove dead units
        self.units.retain(|u| u.hp > 0);
        true
    }

    fn outcome(&self) -> i32 {
        self.units.iter().map(|u| u.hp).sum()
    }
}

fn part1(input: &str) -> i32 {
    let mut state = State::from_input(input, 3);
    let mut rounds = 0;

    loop {
        if !state.simulate_round() {
            break;
        }
        rounds += 1;
    }

    rounds * state.outcome()
}

fn part2(input: &str) -> i32 {
    for elf_attack in 4..=200 {
        let mut state = State::from_input(input, elf_attack);
        let initial_elf_count = state.units.iter().filter(|u| u.unit_type == UnitType::Elf).count();
        let mut rounds = 0;

        loop {
            if !state.simulate_round() {
                break;
            }
            rounds += 1;
            
            // Check if any elf died
            let current_elf_count = state.units.iter().filter(|u| u.unit_type == UnitType::Elf).count();
            if current_elf_count < initial_elf_count {
                break; // An elf died, try next attack power
            }
        }

        let final_elf_count = state.units.iter().filter(|u| u.unit_type == UnitType::Elf).count();
        if final_elf_count == initial_elf_count {
            return rounds * state.outcome();
        }
    }

    0
}

fn main() {
    let input = read_input(15);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample1() {
        let input = "\
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        assert_eq!(part1(input), 27730);
    }

    #[test]
    fn test_part1_sample2() {
        let input = "\
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";
        assert_eq!(part1(input), 36334);
    }

    #[test]
    fn test_part1_sample3() {
        let input = "\
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";
        assert_eq!(part1(input), 39514);
    }

    #[test]
    fn test_part1_sample4() {
        let input = "\
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";
        assert_eq!(part1(input), 27755);
    }

    #[test]
    fn test_part1_sample5() {
        let input = "\
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";
        assert_eq!(part1(input), 28944);
    }

    #[test]
    fn test_part1_sample6() {
        let input = "\
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";
        assert_eq!(part1(input), 18740);
    }
}
