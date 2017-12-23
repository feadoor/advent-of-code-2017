use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

const INPUT_PATH: &'static str = "inputs/input.txt";

enum Value {
    Register(String),
    Direct(i64),
}

enum Instruction {
    Set(String, Value),
    Sub(String, Value),
    Mul(String, Value),
    Jnz(Value, Value),
}

struct ProgramRunner<'a> {
    program: &'a [Instruction],
    current_instruction: i64,
    registers: HashMap<String, i64>,
    multiply_count: usize,
}

impl<'a> ProgramRunner<'a> {
    fn new(program: &'a [Instruction]) -> ProgramRunner<'a> {
        ProgramRunner {
            program: program,
            current_instruction: 0,
            registers: HashMap::new(),
            multiply_count: 0,
        }
    }

    fn get_register_mut(&mut self, name: &str) -> &mut i64 {
        self.registers.entry(name.to_string()).or_insert(0)
    }

    fn get_value(&self, val: &Value) -> i64 {
        match *val {
            Value::Register(ref name) => *self.registers.get(name).unwrap_or(&0),
            Value::Direct(amount) => amount,
        }
    }

    fn in_bounds(&self) -> bool {
        self.current_instruction >= 0 &&
        self.current_instruction < self.program.len() as i64
    }

    fn step(&mut self) -> bool {
        use Instruction::*;

        if self.in_bounds() {
            let old_instruction = self.current_instruction;
            match self.program[self.current_instruction as usize] {
                Set(ref reg, ref val) => *self.get_register_mut(reg) = self.get_value(val),
                Sub(ref reg, ref val) => *self.get_register_mut(reg) -= self.get_value(val),
                Mul(ref reg, ref val) => {
                    *self.get_register_mut(reg) *= self.get_value(val);
                    self.multiply_count += 1;
                },
                Jnz(ref val1, ref val2) => if self.get_value(val1) != 0 { self.current_instruction += self.get_value(val2); }
            }
            if self.current_instruction == old_instruction {
                self.current_instruction += 1;
            }

            return true;
         }

         false
    }
}

fn parse_instruction(instruction_str: &str) -> Instruction {
    let mut words = instruction_str.split(' ');
    let instr_type = words.next().unwrap();

    macro_rules! next_value {
        () => (parse_value(words.next().unwrap()))
    }

    macro_rules! next_register {
        () => (words.next().unwrap().to_string())
    }

    match instr_type {
        "set" => Instruction::Set(next_register!(), next_value!()),
        "sub" => Instruction::Sub(next_register!(), next_value!()),
        "mul" => Instruction::Mul(next_register!(), next_value!()),
        "jnz" => Instruction::Jnz(next_value!(), next_value!()),
        x => panic!("Unexpected instruction {}", x),
    }
}

fn parse_value(value_str: &str) -> Value {
    if value_str.chars().all(|c| c.is_alphabetic()) {
        Value::Register(value_str.to_string())
    } else {
        Value::Direct(value_str.parse().unwrap())
    }
}

fn get_program_from_file(path: &str) -> Vec<Instruction> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    reader.lines()
        .map(|line| parse_instruction(&line.unwrap()))
        .collect()
}

fn is_prime(n: u64) -> bool {
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1
    }

    true
}

fn part1() {
    let program = get_program_from_file(INPUT_PATH);
    let mut runner = ProgramRunner::new(&program);
    while runner.step() { }
    println!("The answer to Part 1 is {}", runner.multiply_count);
}

fn part2() {
    let b = 108400;
    let c = 125400;
    let non_primes = (b..c + 1).filter(|&p| p % 17 == b % 17 && !is_prime(p)).count();
    println!("The answer to Part 2 is {}", non_primes);
}

fn main() {
    part1();
    part2();
}
