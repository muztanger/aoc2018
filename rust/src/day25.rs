use aoc2018::read_input;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
            + (self.w - other.w).abs()
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);

        if px == py {
            return;
        }

        if self.rank[px] < self.rank[py] {
            self.parent[px] = py;
        } else if self.rank[px] > self.rank[py] {
            self.parent[py] = px;
        } else {
            self.parent[py] = px;
            self.rank[px] += 1;
        }
    }

    fn count_sets(&mut self) -> usize {
        let n = self.parent.len();
        (0..n).map(|i| self.find(i)).collect::<std::collections::HashSet<_>>().len()
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<i32> = line.trim().split(',').map(|s| s.trim().parse().unwrap()).collect();
            Point {
                x: parts[0],
                y: parts[1],
                z: parts[2],
                w: parts[3],
            }
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let points = parse_input(input);
    let n = points.len();
    let mut uf = UnionFind::new(n);

    for i in 0..n {
        for j in i + 1..n {
            if points[i].manhattan_distance(&points[j]) <= 3 {
                uf.union(i, j);
            }
        }
    }

    uf.count_sets()
}

fn part2(_input: &str) -> &str {
    "Merry Christmas!"
}

fn main() {
    let input = read_input(25);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = " 0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0";
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part1_example2() {
        let input = "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0";
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn test_part1_example3() {
        let input = "1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2";
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_part1_example4() {
        let input = "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2";
        assert_eq!(part1(input), 8);
    }
}
