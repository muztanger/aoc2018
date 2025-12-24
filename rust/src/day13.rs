use aoc2018::read_input;
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn_left(self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

#[derive(Clone, Debug)]
struct Cart {
    x: i32,
    y: i32,
    dir: Dir,
    turn_state: u8, // 0: left, 1: straight, 2: right
    alive: bool,
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Cart>) {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut carts = Vec::new();

    for (y, row) in grid.iter_mut().enumerate() {
        for (x, ch) in row.iter_mut().enumerate() {
            let dir = match *ch {
                '^' => Some(Dir::Up),
                'v' => Some(Dir::Down),
                '<' => Some(Dir::Left),
                '>' => Some(Dir::Right),
                _ => None,
            };

            if let Some(d) = dir {
                let track_under = match d {
                    Dir::Up | Dir::Down => '|',
                    Dir::Left | Dir::Right => '-',
                };
                *ch = track_under;
                carts.push(Cart {
                    x: x as i32,
                    y: y as i32,
                    dir: d,
                    turn_state: 0,
                    alive: true,
                });
            }
        }
    }

    (grid, carts)
}

fn step_cart(cart: &mut Cart, grid: &[Vec<char>]) {
    match cart.dir {
        Dir::Up => cart.y -= 1,
        Dir::Down => cart.y += 1,
        Dir::Left => cart.x -= 1,
        Dir::Right => cart.x += 1,
    }

    let track = grid[cart.y as usize][cart.x as usize];
    match track {
        '/' => {
            cart.dir = match cart.dir {
                Dir::Up => Dir::Right,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Down,
                Dir::Right => Dir::Up,
            };
        }
        '\\' => {
            cart.dir = match cart.dir {
                Dir::Up => Dir::Left,
                Dir::Down => Dir::Right,
                Dir::Left => Dir::Up,
                Dir::Right => Dir::Down,
            };
        }
        '+' => {
            cart.dir = match cart.turn_state {
                0 => cart.dir.turn_left(),
                1 => cart.dir, // straight
                _ => cart.dir.turn_right(),
            };
            cart.turn_state = (cart.turn_state + 1) % 3;
        }
        '-' | '|' => {}
        other => panic!("Unexpected track char: {}", other),
    }
}

fn simulate(grid: &[Vec<char>], mut carts: Vec<Cart>, stop_on_first_collision: bool) -> (Option<(i32, i32)>, Option<(i32, i32)>) {
    let mut first_collision = None;

    loop {
        carts.sort_by(|a, b| match a.y.cmp(&b.y) {
            Ordering::Equal => a.x.cmp(&b.x),
            other => other,
        });

        for i in 0..carts.len() {
            if !carts[i].alive {
                continue;
            }

            step_cart(&mut carts[i], grid);

            for j in 0..carts.len() {
                if i == j || !carts[j].alive {
                    continue;
                }
                if carts[i].x == carts[j].x && carts[i].y == carts[j].y {
                    if first_collision.is_none() {
                        first_collision = Some((carts[i].x, carts[i].y));
                    }
                    carts[i].alive = false;
                    carts[j].alive = false;
                    if stop_on_first_collision {
                        return (first_collision, None);
                    }
                    break;
                }
            }
        }

        let alive: Vec<&Cart> = carts.iter().filter(|c| c.alive).collect();
        if alive.len() <= 1 {
            let last = alive.first().map(|c| (c.x, c.y));
            return (first_collision, last);
        }
    }
}

fn part1(input: &str) -> String {
    let (grid, carts) = parse(input);
    let (first, _) = simulate(&grid, carts, true);
    let (x, y) = first.expect("No collision found");
    format!("{},{}", x, y)
}

fn part2(input: &str) -> String {
    let (grid, carts) = parse(input);
    let (_, last) = simulate(&grid, carts, false);
    let (x, y) = last.expect("No cart left");
    format!("{},{}", x, y)
}

fn main() {
    let input = read_input(13);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "\
/->-\\        
|   |  /----\\
| /-+--+-\\  |
| | |  | v  |
\\-+-/  \\-+--/
  \\------/   ";

    const SAMPLE2: &str = "\
/>-<\\  
|   |  
| /<+-\\
| | | v
\\>+</ |
  |   ^
  \\<->/";

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(SAMPLE1), "7,3");
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(SAMPLE2), "6,4");
    }
}
