use aoc2018::read_input;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegionType {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Tool {
    Torch = 0,
    ClimbingGear = 1,
    Neither = 2,
}

struct Cave {
    depth: i32,
    target: (i32, i32),
    erosion_cache: HashMap<(i32, i32), i32>,
}

impl Cave {
    fn new(depth: i32, target: (i32, i32)) -> Self {
        Cave {
            depth,
            target,
            erosion_cache: HashMap::new(),
        }
    }
    
    fn geologic_index(&mut self, x: i32, y: i32) -> i32 {
        if (x, y) == (0, 0) || (x, y) == self.target {
            0
        } else if y == 0 {
            x * 16807
        } else if x == 0 {
            y * 48271
        } else {
            let erosion_left = self.erosion_level(x - 1, y);
            let erosion_up = self.erosion_level(x, y - 1);
            erosion_left * erosion_up
        }
    }
    
    fn erosion_level(&mut self, x: i32, y: i32) -> i32 {
        if let Some(&level) = self.erosion_cache.get(&(x, y)) {
            return level;
        }
        
        let geo_index = self.geologic_index(x, y);
        let erosion = (geo_index + self.depth) % 20183;
        self.erosion_cache.insert((x, y), erosion);
        erosion
    }
    
    fn region_type(&mut self, x: i32, y: i32) -> RegionType {
        match self.erosion_level(x, y) % 3 {
            0 => RegionType::Rocky,
            1 => RegionType::Wet,
            2 => RegionType::Narrow,
            _ => unreachable!(),
        }
    }
    
    fn valid_tools(&mut self, x: i32, y: i32) -> Vec<Tool> {
        match self.region_type(x, y) {
            RegionType::Rocky => vec![Tool::ClimbingGear, Tool::Torch],
            RegionType::Wet => vec![Tool::ClimbingGear, Tool::Neither],
            RegionType::Narrow => vec![Tool::Torch, Tool::Neither],
        }
    }
}

fn parse_input(input: &str) -> (i32, (i32, i32)) {
    let lines: Vec<&str> = input.lines().collect();
    let depth = lines[0].strip_prefix("depth: ").unwrap().parse().unwrap();
    let target_parts: Vec<i32> = lines[1]
        .strip_prefix("target: ")
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    (depth, (target_parts[0], target_parts[1]))
}

fn part1(input: &str) -> i32 {
    let (depth, target) = parse_input(input);
    let mut cave = Cave::new(depth, target);
    
    let mut risk = 0;
    for y in 0..=target.1 {
        for x in 0..=target.0 {
            risk += cave.region_type(x, y) as i32;
        }
    }
    risk
}

fn part2(input: &str) -> i32 {
    let (depth, target) = parse_input(input);
    let mut cave = Cave::new(depth, target);
    
    // Dijkstra's algorithm: (time, x, y, tool)
    let mut heap: BinaryHeap<Reverse<(i32, i32, i32, Tool)>> = BinaryHeap::new();
    let mut visited: HashMap<(i32, i32, Tool), i32> = HashMap::new();
    
    heap.push(Reverse((0, 0, 0, Tool::Torch)));
    
    while let Some(Reverse((time, x, y, tool))) = heap.pop() {
        if (x, y) == target && tool == Tool::Torch {
            return time;
        }
        
        if let Some(&prev_time) = visited.get(&(x, y, tool)) {
            if time >= prev_time {
                continue;
            }
        }
        visited.insert((x, y, tool), time);
        
        // Try switching tools (7 minutes)
        let valid_tools = cave.valid_tools(x, y);
        for &new_tool in &valid_tools {
            if new_tool != tool {
                let new_time = time + 7;
                if visited.get(&(x, y, new_tool)).map_or(true, |&t| new_time < t) {
                    heap.push(Reverse((new_time, x, y, new_tool)));
                }
            }
        }
        
        // Try moving to adjacent regions (1 minute)
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = x + dx;
            let ny = y + dy;
            
            if nx >= 0 && ny >= 0 && nx <= target.0 + 100 && ny <= target.1 + 100 {
                let valid_tools = cave.valid_tools(nx, ny);
                if valid_tools.contains(&tool) {
                    let new_time = time + 1;
                    if visited.get(&(nx, ny, tool)).map_or(true, |&t| new_time < t) {
                        heap.push(Reverse((new_time, nx, ny, tool)));
                    }
                }
            }
        }
    }
    
    -1 // Should never reach here
}

fn main() {
    let input = read_input(22);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "depth: 510\ntarget: 10,10";
        assert_eq!(part1(input), 114);
    }

    #[test]
    fn test_part2() {
        let input = "depth: 510\ntarget: 10,10";
        assert_eq!(part2(input), 45);
    }
}
