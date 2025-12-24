use aoc2018::read_input;

fn power_level(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += serial;
    power *= rack_id;
    power = (power / 100) % 10;
    power - 5
}

// Build a summed-area table for efficient range sum queries
fn build_sat(serial: i32) -> Vec<Vec<i32>> {
    let mut sat = vec![vec![0; 301]; 301];
    
    for y in 1..=300 {
        for x in 1..=300 {
            let cell_power = power_level(x, y, serial);
            sat[y as usize][x as usize] = cell_power 
                + sat[(y-1) as usize][x as usize] 
                + sat[y as usize][(x-1) as usize] 
                - sat[(y-1) as usize][(x-1) as usize];
        }
    }
    
    sat
}

// Get sum of square with top-left at (x, y) and size `size`
fn square_sum(sat: &Vec<Vec<i32>>, x: i32, y: i32, size: i32) -> i32 {
    let x1 = x - 1;
    let y1 = y - 1;
    let x2 = x + size - 1;
    let y2 = y + size - 1;
    
    sat[y2 as usize][x2 as usize] 
        - sat[y1 as usize][x2 as usize] 
        - sat[y2 as usize][x1 as usize] 
        + sat[y1 as usize][x1 as usize]
}

fn part1(input: &str) -> String {
    let serial: i32 = input.trim().parse().unwrap();
    let sat = build_sat(serial);
    
    let mut max_power = i32::MIN;
    let mut best_x = 0;
    let mut best_y = 0;
    
    for y in 1..=298 {
        for x in 1..=298 {
            let power = square_sum(&sat, x, y, 3);
            if power > max_power {
                max_power = power;
                best_x = x;
                best_y = y;
            }
        }
    }
    
    format!("{},{}", best_x, best_y)
}

fn part2(input: &str) -> String {
    let serial: i32 = input.trim().parse().unwrap();
    let sat = build_sat(serial);
    
    let mut max_power = i32::MIN;
    let mut best_x = 0;
    let mut best_y = 0;
    let mut best_size = 0;
    
    for size in 1..=300 {
        for y in 1..=(301-size) {
            for x in 1..=(301-size) {
                let power = square_sum(&sat, x, y, size);
                if power > max_power {
                    max_power = power;
                    best_x = x;
                    best_y = y;
                    best_size = size;
                }
            }
        }
    }
    
    format!("{},{},{}", best_x, best_y, best_size)
}

fn main() {
    let input = read_input(11);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("18"), "33,45");
        assert_eq!(part1("42"), "21,61");
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2("18"), "90,269,16");
        assert_eq!(part2("42"), "232,251,12");
    }
}
