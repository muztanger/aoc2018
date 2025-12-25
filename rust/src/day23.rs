use aoc2018::read_input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug, Clone, Copy)]
struct Nanobot {
    x: i64,
    y: i64,
    z: i64,
    r: i64,
}

impl Nanobot {
    fn manhattan_distance(&self, x: i64, y: i64, z: i64) -> i64 {
        (self.x - x).abs() + (self.y - y).abs() + (self.z - z).abs()
    }
    
    fn in_range(&self, other: &Nanobot) -> bool {
        self.manhattan_distance(other.x, other.y, other.z) <= self.r
    }
}

fn parse_input(input: &str) -> Vec<Nanobot> {
    input
        .lines()
        .map(|line| {
            // pos=<x,y,z>, r=radius
            let parts: Vec<&str> = line.split(", ").collect();
            let pos = parts[0].strip_prefix("pos=<").unwrap().strip_suffix(">").unwrap();
            let coords: Vec<i64> = pos.split(',').map(|s| s.parse().unwrap()).collect();
            let r = parts[1].strip_prefix("r=").unwrap().parse().unwrap();
            
            Nanobot {
                x: coords[0],
                y: coords[1],
                z: coords[2],
                r,
            }
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let nanobots = parse_input(input);
    
    // Find the nanobot with the largest signal radius
    let strongest = nanobots.iter().max_by_key(|bot| bot.r).unwrap();
    
    // Count how many nanobots are in range of the strongest
    nanobots.iter().filter(|bot| strongest.in_range(bot)).count()
}

fn part2(input: &str) -> i64 {
    let nanobots = parse_input(input);
    
    // Find the bounding box
    let min_x = nanobots.iter().map(|bot| bot.x).min().unwrap();
    let max_x = nanobots.iter().map(|bot| bot.x).max().unwrap();
    let min_y = nanobots.iter().map(|bot| bot.y).min().unwrap();
    let max_y = nanobots.iter().map(|bot| bot.y).max().unwrap();
    let min_z = nanobots.iter().map(|bot| bot.z).min().unwrap();
    let max_z = nanobots.iter().map(|bot| bot.z).max().unwrap();
    
    // Start with a large cube and subdivide
    let mut best_count = 0;
    let mut best_distance = i64::MAX;
    
    // Priority queue: (-count, distance, size, x, y, z)
    let mut queue: BinaryHeap<(i64, i64, i64, i64, i64, i64)> = BinaryHeap::new();
    
    let size = (max_x - min_x).max(max_y - min_y).max(max_z - min_z);
    let init_count = count_in_range(&nanobots, min_x, min_y, min_z, size);
    queue.push((init_count, 0, size, min_x, min_y, min_z));
    
    while let Some((count, _, size, x, y, z)) = queue.pop() {
        if count < best_count {
            continue;
        }
        
        if size == 0 {
            let dist = x.abs() + y.abs() + z.abs();
            if count > best_count || (count == best_count && dist < best_distance) {
                best_count = count;
                best_distance = dist;
            }
            continue;
        }
        
        let new_size = size / 2;
        
        for dx in 0..=1 {
            for dy in 0..=1 {
                for dz in 0..=1 {
                    let nx = x + dx * new_size;
                    let ny = y + dy * new_size;
                    let nz = z + dz * new_size;
                    
                    let n_count = count_in_range(&nanobots, nx, ny, nz, new_size);
                    
                    if n_count >= best_count {
                        let dist = nx.abs() + ny.abs() + nz.abs();
                        queue.push((n_count, Reverse(dist).0, new_size, nx, ny, nz));
                    }
                }
            }
        }
    }
    
    best_distance
}

fn count_in_range(nanobots: &[Nanobot], x: i64, y: i64, z: i64, size: i64) -> i64 {
    let mut count = 0;
    for bot in nanobots {
        // Check if the cube intersects with the nanobot's range
        let dx = if x > bot.x {
            x - bot.x
        } else if x + size < bot.x {
            bot.x - (x + size)
        } else {
            0
        };
        
        let dy = if y > bot.y {
            y - bot.y
        } else if y + size < bot.y {
            bot.y - (y + size)
        } else {
            0
        };
        
        let dz = if z > bot.z {
            z - bot.z
        } else if z + size < bot.z {
            bot.z - (z + size)
        } else {
            0
        };
        
        if dx + dy + dz <= bot.r {
            count += 1;
        }
    }
    count
}

fn main() {
    let input = read_input(23);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";
        assert_eq!(part1(input), 7);
    }

    #[test]
    fn test_part2() {
        let input = "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";
        assert_eq!(part2(input), 36);
    }
}
