use aoc2018::read_input;
use std::collections::HashSet;

struct Grid {
    clay: HashSet<(i32, i32)>,
    water_flow: HashSet<(i32, i32)>,
    water_rest: HashSet<(i32, i32)>,
    min_y: i32,
    max_y: i32,
}

impl Grid {
    fn new(clay: HashSet<(i32, i32)>) -> Self {
        let min_y = clay.iter().map(|(_, y)| *y).min().unwrap();
        let max_y = clay.iter().map(|(_, y)| *y).max().unwrap();
        
        Grid {
            clay,
            water_flow: HashSet::new(),
            water_rest: HashSet::new(),
            min_y,
            max_y,
        }
    }
    
    fn is_blocked(&self, x: i32, y: i32) -> bool {
        self.clay.contains(&(x, y)) || self.water_rest.contains(&(x, y))
    }
    
    fn flow(&mut self, x: i32, y: i32) {
        if y > self.max_y || self.water_flow.contains(&(x, y)) {
            return;
        }
        
        if self.is_blocked(x, y) {
            return;
        }
        
        self.water_flow.insert((x, y));
        
        // Try flowing down
        if !self.is_blocked(x, y + 1) {
            self.flow(x, y + 1);
        }
        
        // If we can't flow down anymore, try spreading horizontally
        if self.is_blocked(x, y + 1) {
            // Spread left
            let mut left_x = x;
            while !self.is_blocked(left_x - 1, y) && self.is_blocked(left_x - 1, y + 1) {
                left_x -= 1;
                self.water_flow.insert((left_x, y));
            }
            
            // Spread right  
            let mut right_x = x;
            while !self.is_blocked(right_x + 1, y) && self.is_blocked(right_x + 1, y + 1) {
                right_x += 1;
                self.water_flow.insert((right_x, y));
            }
            
            // Check if water is contained (walls on both sides)
            let left_wall = self.is_blocked(left_x - 1, y);
            let right_wall = self.is_blocked(right_x + 1, y);
            
            if left_wall && right_wall {
                // Water settles - convert flow to rest
                for i in left_x..=right_x {
                    self.water_flow.remove(&(i, y));
                    self.water_rest.insert((i, y));
                }
            } else {
                // Water can flow off one or both sides
                if !left_wall && !self.is_blocked(left_x - 1, y + 1) {
                    self.flow(left_x - 1, y);
                }
                if !right_wall && !self.is_blocked(right_x + 1, y + 1) {
                    self.flow(right_x + 1, y);
                }
            }
        }
    }
    
    fn count_water(&self) -> usize {
        self.water_flow.iter()
            .chain(self.water_rest.iter())
            .filter(|(_, y)| *y >= self.min_y && *y <= self.max_y)
            .count()
    }
    
    fn count_resting_water(&self) -> usize {
        self.water_rest.iter()
            .filter(|(_, y)| *y >= self.min_y && *y <= self.max_y)
            .count()
    }
}

fn parse_input(input: &str) -> HashSet<(i32, i32)> {
    let mut clay = HashSet::new();
    
    for line in input.lines() {
        let parts: Vec<&str> = line.split(", ").collect();
        
        let (first_var, first_val) = parts[0].split_once('=').unwrap();
        let first_val: i32 = first_val.parse().unwrap();
        
        let (_, range_str) = parts[1].split_once('=').unwrap();
        let range_parts: Vec<&str> = range_str.split("..").collect();
        let start: i32 = range_parts[0].parse().unwrap();
        let end: i32 = range_parts[1].parse().unwrap();
        
        for i in start..=end {
            let (x, y) = if first_var == "x" {
                (first_val, i)
            } else {
                (i, first_val)
            };
            clay.insert((x, y));
        }
    }
    
    clay
}

fn part1(input: &str) -> usize {
    let clay = parse_input(input);
    let mut grid = Grid::new(clay);
    
    // Start water flow from spring at (500, 0)
    grid.flow(500, 0);
    
    grid.count_water()
}

fn part2(input: &str) -> usize {
    let clay = parse_input(input);
    let mut grid = Grid::new(clay);
    
    // Start water flow from spring at (500, 0)
    grid.flow(500, 0);
    
    grid.count_resting_water()
}

fn main() {
    let input = read_input(17);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";
        assert_eq!(part1(input), 57);
    }

    #[test]
    fn test_part2() {
        let input = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";
        assert_eq!(part2(input), 29);
    }
}
