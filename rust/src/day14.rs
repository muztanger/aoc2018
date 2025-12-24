use aoc2018::read_input;

fn make_recipes(target_len: usize) -> Vec<u8> {
    let mut recipes: Vec<u8> = vec![3, 7];
    let mut elf1 = 0usize;
    let mut elf2 = 1usize;

    while recipes.len() < target_len {
        let sum = recipes[elf1] + recipes[elf2];
        if sum >= 10 {
            recipes.push(1);
            recipes.push(sum - 10);
        } else {
            recipes.push(sum);
        }

        elf1 = (elf1 + 1 + recipes[elf1] as usize) % recipes.len();
        elf2 = (elf2 + 1 + recipes[elf2] as usize) % recipes.len();
    }

    recipes
}

fn part1(input: &str) -> String {
    let n: usize = input.trim().parse().unwrap();
    let target_len = n + 10;
    let recipes = make_recipes(target_len);
    recipes[n..n + 10]
        .iter()
        .map(|d| (b'0' + *d) as char)
        .collect()
}

fn part2(input: &str) -> usize {
    let pattern: Vec<u8> = input.trim().bytes().map(|b| b - b'0').collect();
    let m = pattern.len();

    let mut recipes: Vec<u8> = vec![3, 7];
    let mut elf1 = 0usize;
    let mut elf2 = 1usize;

    loop {
        let sum = recipes[elf1] + recipes[elf2];
        if sum >= 10 {
            recipes.push(1);
            recipes.push(sum - 10);
        } else {
            recipes.push(sum);
        }

        elf1 = (elf1 + 1 + recipes[elf1] as usize) % recipes.len();
        elf2 = (elf2 + 1 + recipes[elf2] as usize) % recipes.len();

        // Check for pattern at the end or one before the end (since we can add two digits)
        let len = recipes.len();
        if len >= m {
            if recipes[len - m..len] == pattern {
                return len - m;
            }
        }
        if len >= m + 1 {
            if recipes[len - m - 1..len - 1] == pattern {
                return len - m - 1;
            }
        }
    }
}

fn main() {
    let input = read_input(14);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_samples() {
        assert_eq!(part1("9"), "5158916779");
        assert_eq!(part1("5"), "0124515891");
        assert_eq!(part1("18"), "9251071085");
        assert_eq!(part1("2018"), "5941429882");
    }

    #[test]
    fn test_part2_samples() {
        assert_eq!(part2("51589"), 9);
        assert_eq!(part2("01245"), 5);
        assert_eq!(part2("92510"), 18);
        assert_eq!(part2("59414"), 2018);
    }
}
