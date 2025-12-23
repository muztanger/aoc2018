use aoc2018::read_input;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = read_input(6);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input, 10000));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            Point {
                x: parts[0].trim().parse().unwrap(),
                y: parts[1].trim().parse().unwrap(),
            }
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let points = parse_input(input);
    
    // Find the bounding box
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();
    
    // Track which points have infinite areas (touch the boundary)
    let mut infinite_areas = HashSet::new();
    
    // Count area for each point
    let mut areas: HashMap<usize, usize> = HashMap::new();
    
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let current = Point { x, y };
            
            // Find the closest point
            let mut min_dist = i32::MAX;
            let mut closest_idx = None;
            let mut tie = false;
            
            for (idx, point) in points.iter().enumerate() {
                let dist = current.manhattan_distance(point);
                if dist < min_dist {
                    min_dist = dist;
                    closest_idx = Some(idx);
                    tie = false;
                } else if dist == min_dist {
                    tie = true;
                }
            }
            
            // If not a tie, count this location for the closest point
            if !tie {
                if let Some(idx) = closest_idx {
                    *areas.entry(idx).or_insert(0) += 1;
                    
                    // If on the boundary, this area is infinite
                    if x == min_x || x == max_x || y == min_y || y == max_y {
                        infinite_areas.insert(idx);
                    }
                }
            }
        }
    }
    
    // Find the largest finite area
    areas
        .iter()
        .filter(|(idx, _)| !infinite_areas.contains(idx))
        .map(|(_, &area)| area)
        .max()
        .unwrap_or(0)
}

fn part2(input: &str, max_distance: i32) -> usize {
    let points = parse_input(input);
    
    // Find the bounding box (expand a bit to be safe)
    let min_x = points.iter().map(|p| p.x).min().unwrap() - 100;
    let max_x = points.iter().map(|p| p.x).max().unwrap() + 100;
    let min_y = points.iter().map(|p| p.y).min().unwrap() - 100;
    let max_y = points.iter().map(|p| p.y).max().unwrap() + 100;
    
    let mut safe_region_size = 0;
    
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let current = Point { x, y };
            
            // Calculate total distance to all points
            let total_distance: i32 = points
                .iter()
                .map(|point| current.manhattan_distance(point))
                .sum();
            
            if total_distance < max_distance {
                safe_region_size += 1;
            }
        }
    }
    
    safe_region_size
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 17);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT, 32), 16);
    }
}
