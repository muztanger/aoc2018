use aoc2018::read_input;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Point {
    fn step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }

    fn step_back(&mut self) {
        self.x -= self.vx;
        self.y -= self.vy;
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            // position=< 10775, -31651> velocity=<-1,  3>
            let parts: Vec<&str> = line.split(&['<', '>', ','][..]).collect();
            Point {
                x: parts[1].trim().parse().unwrap(),
                y: parts[2].trim().parse().unwrap(),
                vx: parts[4].trim().parse().unwrap(),
                vy: parts[5].trim().parse().unwrap(),
            }
        })
        .collect()
}

fn bounding_box(points: &[Point]) -> (i32, i32, i32, i32) {
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();
    (min_x, max_x, min_y, max_y)
}

fn area_size(points: &[Point]) -> i64 {
    let (min_x, max_x, min_y, max_y) = bounding_box(points);
    (max_x - min_x) as i64 * (max_y - min_y) as i64
}

fn display_points(points: &[Point]) -> String {
    let (min_x, max_x, min_y, max_y) = bounding_box(points);
    let point_set: HashSet<(i32, i32)> = points.iter().map(|p| (p.x, p.y)).collect();
    
    let mut result = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if point_set.contains(&(x, y)) {
                result.push('#');
            } else {
                result.push('.');
            }
        }
        result.push('\n');
    }
    result
}

fn solve(input: &str) -> (String, i32) {
    let mut points = parse_input(input);
    let mut seconds = 0;
    let mut prev_area = area_size(&points);
    
    // Simulate until the area starts increasing (points diverge)
    loop {
        for point in points.iter_mut() {
            point.step();
        }
        seconds += 1;
        
        let current_area = area_size(&points);
        if current_area > prev_area {
            // We've gone too far, step back one
            for point in points.iter_mut() {
                point.step_back();
            }
            seconds -= 1;
            break;
        }
        prev_area = current_area;
    }
    
    (display_points(&points), seconds)
}

fn part1(input: &str) -> String {
    solve(input).0
}

fn part2(input: &str) -> i32 {
    solve(input).1
}

fn main() {
    let input = read_input(10);

    let (message, seconds) = solve(&input);
    println!("Part 1:\n{}", message);
    println!("Part 2: {}", seconds);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 3);
    }
    
    #[test]
    fn test_display() {
        let message = part1(TEST_INPUT);
        println!("{}", message);
        // Should spell "HI"
        assert!(message.contains("#...#..###"));
    }
}
