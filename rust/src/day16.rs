use aoc2018::read_input;
use std::collections::{HashMap, HashSet};

type Registers = [usize; 4];

#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug, Clone)]
struct Sample {
    before: Registers,
    instruction: Instruction,
    after: Registers,
}

fn parse_input(input: &str) -> (Vec<Sample>, Vec<Instruction>) {
    let parts: Vec<&str> = input.split("\n\n\n").collect();
    
    let samples = parts[0]
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|sample| {
            let lines: Vec<&str> = sample.lines().collect();
            let before_str = lines[0].trim_start_matches("Before: [").trim_end_matches(']');
            let before: Vec<usize> = before_str.split(", ").map(|n| n.parse().unwrap()).collect();
            
            let inst_parts: Vec<usize> = lines[1].split_whitespace().map(|n| n.parse().unwrap()).collect();
            let instruction = Instruction {
                opcode: inst_parts[0],
                a: inst_parts[1],
                b: inst_parts[2],
                c: inst_parts[3],
            };
            
            let after_str = lines[2].trim_start_matches("After:  [").trim_end_matches(']');
            let after: Vec<usize> = after_str.split(", ").map(|n| n.parse().unwrap()).collect();
            
            Sample {
                before: [before[0], before[1], before[2], before[3]],
                instruction,
                after: [after[0], after[1], after[2], after[3]],
            }
        })
        .collect();
    
    let test_program = if parts.len() > 1 {
        parts[1]
            .lines()
            .filter(|l| !l.is_empty())
            .map(|line| {
                let parts: Vec<usize> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();
                Instruction {
                    opcode: parts[0],
                    a: parts[1],
                    b: parts[2],
                    c: parts[3],
                }
            })
            .collect()
    } else {
        Vec::new()
    };
    
    (samples, test_program)
}

fn execute(opcode: &str, regs: &mut Registers, a: usize, b: usize, c: usize) {
    match opcode {
        "addr" => regs[c] = regs[a] + regs[b],
        "addi" => regs[c] = regs[a] + b,
        "mulr" => regs[c] = regs[a] * regs[b],
        "muli" => regs[c] = regs[a] * b,
        "banr" => regs[c] = regs[a] & regs[b],
        "bani" => regs[c] = regs[a] & b,
        "borr" => regs[c] = regs[a] | regs[b],
        "bori" => regs[c] = regs[a] | b,
        "setr" => regs[c] = regs[a],
        "seti" => regs[c] = a,
        "gtir" => regs[c] = if a > regs[b] { 1 } else { 0 },
        "gtri" => regs[c] = if regs[a] > b { 1 } else { 0 },
        "gtrr" => regs[c] = if regs[a] > regs[b] { 1 } else { 0 },
        "eqir" => regs[c] = if a == regs[b] { 1 } else { 0 },
        "eqri" => regs[c] = if regs[a] == b { 1 } else { 0 },
        "eqrr" => regs[c] = if regs[a] == regs[b] { 1 } else { 0 },
        _ => panic!("Unknown opcode: {}", opcode),
    }
}

fn get_all_opcodes() -> Vec<&'static str> {
    vec![
        "addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori",
        "setr", "seti", "gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr",
    ]
}

fn matches_sample(sample: &Sample, opcode: &str) -> bool {
    let mut regs = sample.before;
    execute(opcode, &mut regs, sample.instruction.a, sample.instruction.b, sample.instruction.c);
    regs == sample.after
}

fn part1(input: &str) -> usize {
    let (samples, _) = parse_input(input);
    let opcodes = get_all_opcodes();
    
    samples.iter()
        .filter(|sample| {
            let matching_count = opcodes.iter()
                .filter(|&&op| matches_sample(sample, op))
                .count();
            matching_count >= 3
        })
        .count()
}

fn part2(input: &str) -> usize {
    let (samples, test_program) = parse_input(input);
    let opcodes = get_all_opcodes();
    
    // Build a map of opcode number -> possible opcode names
    let mut possible: HashMap<usize, HashSet<&str>> = HashMap::new();
    for i in 0..16 {
        possible.insert(i, opcodes.iter().copied().collect());
    }
    
    // For each sample, eliminate opcodes that don't match
    for sample in &samples {
        let opcode_num = sample.instruction.opcode;
        let matches: HashSet<&str> = opcodes.iter()
            .filter(|&&op| matches_sample(sample, op))
            .copied()
            .collect();
        
        if let Some(poss) = possible.get_mut(&opcode_num) {
            *poss = poss.intersection(&matches).copied().collect();
        }
    }
    
    // Deduce the mapping using constraint propagation
    let mut opcode_map: HashMap<usize, &str> = HashMap::new();
    
    while opcode_map.len() < 16 {
        // Find an opcode number with only one possibility
        let mut found = None;
        for (&num, poss) in &possible {
            if poss.len() == 1 && !opcode_map.contains_key(&num) {
                found = Some((num, *poss.iter().next().unwrap()));
                break;
            }
        }
        
        if let Some((num, op)) = found {
            opcode_map.insert(num, op);
            // Remove this opcode from all other possibilities
            for poss in possible.values_mut() {
                poss.remove(op);
            }
        } else {
            break;
        }
    }
    
    // Execute the test program
    let mut regs = [0, 0, 0, 0];
    for inst in test_program {
        if let Some(&opcode) = opcode_map.get(&inst.opcode) {
            execute(opcode, &mut regs, inst.a, inst.b, inst.c);
        }
    }
    
    regs[0]
}

fn main() {
    let input = read_input(16);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]";
        // This sample matches mulr, addi, and seti (3 opcodes)
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn test_sample_matching() {
        let sample = Sample {
            before: [3, 2, 1, 1],
            instruction: Instruction { opcode: 9, a: 2, b: 1, c: 2 },
            after: [3, 2, 2, 1],
        };
        
        assert!(matches_sample(&sample, "mulr")); // reg[2]=1 * reg[1]=2 = 2
        assert!(matches_sample(&sample, "addi")); // reg[2]=1 + val 1 = 2
        assert!(matches_sample(&sample, "seti")); // val 2 -> reg[2]
    }
}
