use aoc2018::read_input;

type Registers = [i64; 6];

#[derive(Debug, Clone)]
struct Instruction {
    opcode: String,
    a: i64,
    b: i64,
    c: i64,
}

struct Program {
    ip_register: usize,
    instructions: Vec<Instruction>,
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

fn parse_input(input: &str) -> Program {
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
    
    Program {
        ip_register,
        instructions,
    }
}

fn run_program(program: &Program, initial_reg0: i64) -> i64 {
    let mut regs: Registers = [initial_reg0, 0, 0, 0, 0, 0];
    let mut ip: i64 = 0;
    
    while ip >= 0 && (ip as usize) < program.instructions.len() {
        // Write IP to bound register
        regs[program.ip_register] = ip;
        
        // Execute instruction
        let inst = &program.instructions[ip as usize];
        execute(&inst.opcode, &mut regs, inst.a, inst.b, inst.c);
        
        // Read IP from bound register and increment
        ip = regs[program.ip_register] + 1;
    }
    
    regs[0]
}

fn sum_of_divisors(n: i64) -> i64 {
    let mut sum = 0;
    for i in 1..=n {
        if n % i == 0 {
            sum += i;
        }
    }
    sum
}

fn part1(input: &str) -> i64 {
    let program = parse_input(input);
    run_program(&program, 0)
}

fn part2(input: &str) -> i64 {
    let program = parse_input(input);
    
    // The program calculates the sum of divisors of a number stored in register 1.
    // With register 0 starting at 1, the initialization phase builds a much larger
    // target number. Running the full program would take too long, so we:
    // 1. Run the initialization to build the target number in register 1
    // 2. Calculate the sum of divisors directly (what the program does inefficiently)
    
    let mut regs: Registers = [1, 0, 0, 0, 0, 0];
    let mut ip: i64 = 0;
    let mut iterations = 0;
    
    // Run initialization phase (typically completes in < 100 iterations)
    while ip >= 0 && (ip as usize) < program.instructions.len() && iterations < 100 {
        regs[program.ip_register] = ip;
        let inst = &program.instructions[ip as usize];
        execute(&inst.opcode, &mut regs, inst.a, inst.b, inst.c);
        ip = regs[program.ip_register] + 1;
        iterations += 1;
        
        // When IP returns to instruction 1 after initialization, we're done
        if ip == 1 && iterations > 50 {
            break;
        }
    }
    
    // Calculate sum of divisors efficiently
    let target = regs[1];
    sum_of_divisors(target)
}

fn main() {
    let input = read_input(19);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";
        assert_eq!(run_program(&parse_input(input), 0), 6);
    }
}
