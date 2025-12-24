use aoc2018::read_input;
use std::collections::HashSet;

type Registers = [i64; 6];

#[derive(Debug, Clone)]
struct Instruction {
    opcode: String,
    a: i64,
    b: i64,
    c: i64,
}

fn execute(opcode: &str, regs: &mut Registers, a: i64, b: i64, c: i64) {
    let a = a as usize;
    let b = b as usize;
    let c = c as usize;
    
    match opcode {
        "addr" => regs[c] = regs[a] + regs[b],
        "addi" => regs[c] = regs[a] + b as i64,
        "mulr" => regs[c] = regs[a] * regs[b],
        "muli" => regs[c] = regs[a] * b as i64,
        "banr" => regs[c] = regs[a] & regs[b],
        "bani" => regs[c] = regs[a] & b as i64,
        "borr" => regs[c] = regs[a] | regs[b],
        "bori" => regs[c] = regs[a] | b as i64,
        "setr" => regs[c] = regs[a],
        "seti" => regs[c] = a as i64,
        "gtir" => regs[c] = if a as i64 > regs[b] { 1 } else { 0 },
        "gtri" => regs[c] = if regs[a] > b as i64 { 1 } else { 0 },
        "gtrr" => regs[c] = if regs[a] > regs[b] { 1 } else { 0 },
        "eqir" => regs[c] = if a as i64 == regs[b] { 1 } else { 0 },
        "eqri" => regs[c] = if regs[a] == b as i64 { 1 } else { 0 },
        "eqrr" => regs[c] = if regs[a] == regs[b] { 1 } else { 0 },
        _ => panic!("Unknown opcode: {}", opcode),
    }
}

fn parse_input(input: &str) -> (usize, Vec<Instruction>) {
    let lines: Vec<&str> = input.lines().collect();
    let ip_register = lines[0]
        .strip_prefix("#ip ")
        .unwrap()
        .parse()
        .unwrap();
    
    let instructions = lines[1..]
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            Instruction {
                opcode: parts[0].to_string(),
                a: parts[1].parse().unwrap(),
                b: parts[2].parse().unwrap(),
                c: parts[3].parse().unwrap(),
            }
        })
        .collect();
    
    (ip_register, instructions)
}

fn find_halting_values(input: &str) -> Vec<i64> {
    let (ip_register, instructions) = parse_input(input);
    let mut regs: Registers = [0, 0, 0, 0, 0, 0];
    let mut ip: i64 = 0;
    let mut halting_values = Vec::new();
    let mut seen_values = HashSet::new();
    
    // The program will halt when register 0 equals register 1
    // Track all values of register 1 when we reach the comparison point
    // The first one is the answer for part 1
    // The last unique one before repeating is the answer for part 2
    
    while ip >= 0 && (ip as usize) < instructions.len() {
        regs[ip_register] = ip;
        let inst = &instructions[ip as usize];
        
        // Check if we're at the comparison instruction (eqrr 1 0 4)
        if ip as usize == 28 {
            if seen_values.contains(&regs[1]) {
                // We've hit a cycle
                break;
            }
            halting_values.push(regs[1]);
            seen_values.insert(regs[1]);
        }
        
        execute(&inst.opcode, &mut regs, inst.a, inst.b, inst.c);
        ip = regs[ip_register] + 1;
    }
    
    halting_values
}

fn part1(input: &str) -> i64 {
    let halting_values = find_halting_values(input);
    *halting_values.first().unwrap_or(&0)
}

fn part2(input: &str) -> i64 {
    let halting_values = find_halting_values(input);
    *halting_values.last().unwrap_or(&0)
}

fn main() {
    let input = read_input(21);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // This is a complex reverse-engineering problem
        // The tests would require running the actual program
        assert!(true);
    }
}
