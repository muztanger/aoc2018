use aoc2018::read_input;
use std::collections::VecDeque;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum UnitType {
    Elf,
    Goblin,
}

#[derive(Clone, Debug)]
struct Unit {
    pos: (i64, i64),
    unit_type: UnitType,
    hp: i64,
    attack: i64,
}

#[derive(Clone)]
struct State {
    grid: Vec<Vec<char>>,
    units: Vec<Unit>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RoundOutcome {
    Completed,
    Incomplete,
}

impl State {
    fn from_input(input: &str, elf_attack: i64) -> Self {
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
                        pos: (x as i64, y as i64),
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

    fn bfs(&self, start: (i64, i64), targets: Vec<(i64, i64)>) -> Option<(i64, i64)> {
        let mut dist = std::collections::HashMap::new();
        let mut queue = VecDeque::new();
        let (sx, sy) = start;
        
        dist.insert((sx, sy), 0);
        queue.push_back((sx, sy));

        let mut reachable = Vec::new();

        while let Some((x, y)) = queue.pop_front() {
            let d = dist[&(x, y)];

            if targets.contains(&(x, y)) {
                reachable.push(((x, y), d));
            }

            for (dx, dy) in &[(0, -1), (-1, 0), (1, 0), (0, 1)] {
                let (nx, ny) = (x + dx, y + dy);
                if nx >= 0 && nx < self.grid[0].len() as i64 && ny >= 0 && ny < self.grid.len() as i64 {
                    if self.grid[ny as usize][nx as usize] == '.'
                        && !self.units.iter().any(|u| u.pos == (nx, ny) && u.hp > 0)
                        && !dist.contains_key(&(nx, ny))
                    {
                        dist.insert((nx, ny), d + 1);
                        queue.push_back((nx, ny));
                    }
                }
            }
        }

        if reachable.is_empty() {
            return None;
        }

        reachable.sort_by_key(|(_, d)| *d);
        let min_dist = reachable[0].1;
        reachable.retain(|(_, d)| *d == min_dist);
        reachable.sort_by_key(|((x, y), _)| (*y, *x));
        Some(reachable[0].0)
    }

    fn simulate_round(&mut self) -> RoundOutcome {
        // Get order of units at start of round
        let mut unit_order: Vec<usize> = (0..self.units.len()).collect();
        unit_order.sort_by_key(|&i| (self.units[i].pos.1, self.units[i].pos.0));

        for &unit_idx in &unit_order {
            // Skip if this unit is dead
            if self.units[unit_idx].hp <= 0 {
                continue;
            }

            let unit_type = self.units[unit_idx].unit_type;
            let enemies: Vec<usize> = (0..self.units.len())
                .filter(|&i| self.units[i].unit_type != unit_type && self.units[i].hp > 0)
                .collect();

            // If no enemies remain when this unit's turn starts, combat ends
            if enemies.is_empty() {
                return RoundOutcome::Incomplete;
            }

            let pos = self.units[unit_idx].pos;
            let adjacent = vec![
                (pos.0, pos.1 - 1),
                (pos.0 - 1, pos.1),
                (pos.0 + 1, pos.1),
                (pos.0, pos.1 + 1),
            ];

            let adjacent_enemy = enemies.iter().find(|&&e| adjacent.contains(&self.units[e].pos));

            // Move if not already adjacent to an enemy
            if adjacent_enemy.is_none() {
                let targets: Vec<(i64, i64)> = enemies
                    .iter()
                    .flat_map(|&e| {
                        let (ex, ey) = self.units[e].pos;
                        vec![(ex, ey - 1), (ex - 1, ey), (ex + 1, ey), (ex, ey + 1)]
                    })
                    .filter(|(x, y)| {
                        *x >= 0 && *x < self.grid[0].len() as i64 
                        && *y >= 0 && *y < self.grid.len() as i64
                        && self.grid[*y as usize][*x as usize] == '.'
                        && !self.units.iter().any(|u| u.pos == (*x, *y) && u.hp > 0)
                    })
                    .collect();

                if !targets.is_empty() {
                    if let Some(target) = self.bfs(pos, targets) {
                        let next = self.get_next_step(pos, target);
                        self.units[unit_idx].pos = next;
                    }
                }
            }

            // Attack after moving
            let pos = self.units[unit_idx].pos;
            let adjacent = vec![
                (pos.0, pos.1 - 1),
                (pos.0 - 1, pos.1),
                (pos.0 + 1, pos.1),
                (pos.0, pos.1 + 1),
            ];

            if let Some(&target_idx) = enemies
                .iter()
                .filter(|&&e| adjacent.contains(&self.units[e].pos) && self.units[e].hp > 0)
                .min_by_key(|&&e| (self.units[e].hp, self.units[e].pos.1, self.units[e].pos.0))
            {
                self.units[target_idx].hp -= self.units[unit_idx].attack;
            }
        }

        self.units.retain(|u| u.hp > 0);
        RoundOutcome::Completed
    }

    fn get_next_step(&self, from: (i64, i64), to: (i64, i64)) -> (i64, i64) {
        // BFS from target back to source to find distances
        let mut dist = std::collections::HashMap::new();
        let mut queue = VecDeque::new();
        
        dist.insert(to, 0);
        queue.push_back(to);

        while let Some((x, y)) = queue.pop_front() {
            if (x, y) == from {
                break; // Found the source
            }
            
            let d = dist[&(x, y)];
            
            for (dx, dy) in &[(0, -1), (-1, 0), (1, 0), (0, 1)] {
                let (nx, ny) = (x + dx, y + dy);
                if nx >= 0 && nx < self.grid[0].len() as i64 && ny >= 0 && ny < self.grid.len() as i64 {
                    if (self.grid[ny as usize][nx as usize] == '.' || (nx, ny) == from)
                        && !self.units.iter().any(|u| u.pos == (nx, ny) && u.hp > 0 && (nx, ny) != from)
                        && !dist.contains_key(&(nx, ny))
                    {
                        dist.insert((nx, ny), d + 1);
                        queue.push_back((nx, ny));
                    }
                }
            }
        }

        if !dist.contains_key(&from) {
            return from; // No path
        }

        let target_dist = dist[&from];
        
        // Find first adjacent cell in reading order that's one step closer to target
        for (dx, dy) in &[(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let (nx, ny) = (from.0 + dx, from.1 + dy);
            if nx >= 0 && nx < self.grid[0].len() as i64 && ny >= 0 && ny < self.grid.len() as i64 {
                if let Some(&d) = dist.get(&(nx, ny)) {
                    if d == target_dist - 1 {
                        return (nx, ny);
                    }
                }
            }
        }

        from
    }
}

fn part1(input: &str) -> i64 {
    let mut state = State::from_input(input, 3);
    let mut rounds = 0;

    loop {
        match state.simulate_round() {
            RoundOutcome::Completed => rounds += 1,
            RoundOutcome::Incomplete => break,
        }
    }

    let total_hp: i64 = state.units.iter().map(|u| u.hp).sum();
    rounds * total_hp
}

fn part2(input: &str) -> i64 {
    for elf_attack in 4..=200 {
        let mut state = State::from_input(input, elf_attack);
        let initial_elf_count = state.units.iter().filter(|u| u.unit_type == UnitType::Elf).count();
        let mut rounds = 0;

        loop {
            match state.simulate_round() {
                RoundOutcome::Completed => {
                    rounds += 1;
                    let elf_count = state.units.iter().filter(|u| u.unit_type == UnitType::Elf).count();
                    if elf_count < initial_elf_count {
                        break;
                    }
                }
                RoundOutcome::Incomplete => break,
            }
        }

        let elf_count = state.units.iter().filter(|u| u.unit_type == UnitType::Elf).count();
        if elf_count == initial_elf_count {
            let total_hp: i64 = state.units.iter().map(|u| u.hp).sum();
            return rounds * total_hp;
        }
    }

    0
}

fn main() {
    let input = read_input(15);

    let result1 = part1(&input);
    assert!(result1 > 237405, "Part 1 answer should be higher than 237405 (previous lower bound), got {}", result1);
    assert!(result1 < 237915, "Part 1 answer should be lower than 237915 (which was too high), got {}", result1);
    println!("Part 1: {}", result1);
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
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
}
