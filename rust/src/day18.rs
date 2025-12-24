use aoc2018::read_input;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

fn parse_input(input: &str) -> Vec<Vec<Acre>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Acre::Open,
                    '|' => Acre::Trees,
                    '#' => Acre::Lumberyard,
                    _ => panic!("Invalid character: {}", ch),
                })
                .collect()
        })
        .collect()
}

fn count_adjacent(grid: &Vec<Vec<Acre>>, row: usize, col: usize, acre_type: Acre) -> usize {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let r = row as i32;
    let c = col as i32;
    
    let mut count = 0;
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = r + dr;
            let nc = c + dc;
            if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                if grid[nr as usize][nc as usize] == acre_type {
                    count += 1;
                }
            }
        }
    }
    count
}

fn simulate_minute(grid: &Vec<Vec<Acre>>) -> Vec<Vec<Acre>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut new_grid = grid.clone();
    
    for r in 0..rows {
        for c in 0..cols {
            new_grid[r][c] = match grid[r][c] {
                Acre::Open => {
                    if count_adjacent(grid, r, c, Acre::Trees) >= 3 {
                        Acre::Trees
                    } else {
                        Acre::Open
                    }
                }
                Acre::Trees => {
                    if count_adjacent(grid, r, c, Acre::Lumberyard) >= 3 {
                        Acre::Lumberyard
                    } else {
                        Acre::Trees
                    }
                }
                Acre::Lumberyard => {
                    let has_lumberyard = count_adjacent(grid, r, c, Acre::Lumberyard) >= 1;
                    let has_trees = count_adjacent(grid, r, c, Acre::Trees) >= 1;
                    if has_lumberyard && has_trees {
                        Acre::Lumberyard
                    } else {
                        Acre::Open
                    }
                }
            };
        }
    }
    
    new_grid
}

fn count_resource_value(grid: &Vec<Vec<Acre>>) -> usize {
    let mut trees = 0;
    let mut lumberyards = 0;
    
    for row in grid {
        for &acre in row {
            match acre {
                Acre::Trees => trees += 1,
                Acre::Lumberyard => lumberyards += 1,
                _ => {}
            }
        }
    }
    
    trees * lumberyards
}

fn grid_to_string(grid: &Vec<Vec<Acre>>) -> String {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|&acre| match acre {
                    Acre::Open => '.',
                    Acre::Trees => '|',
                    Acre::Lumberyard => '#',
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn part1(input: &str) -> usize {
    let mut grid = parse_input(input);
    
    for _ in 0..10 {
        grid = simulate_minute(&grid);
    }
    
    count_resource_value(&grid)
}

fn part2(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut seen: HashMap<String, usize> = HashMap::new();
    let target = 1_000_000_000;
    
    for minute in 0..target {
        let state = grid_to_string(&grid);
        
        if let Some(&prev_minute) = seen.get(&state) {
            // Found a cycle
            let cycle_length = minute - prev_minute;
            let remaining = target - minute;
            let final_offset = remaining % cycle_length;
            
            // Simulate the remaining steps
            for _ in 0..final_offset {
                grid = simulate_minute(&grid);
            }
            
            return count_resource_value(&grid);
        }
        
        seen.insert(state, minute);
        grid = simulate_minute(&grid);
    }
    
    count_resource_value(&grid)
}

fn main() {
    let input = read_input(18);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";
        assert_eq!(part1(input), 1147);
    }
}
