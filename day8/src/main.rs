extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap};
use std::cmp;
use regex::Regex;

const PART_1_INPUT_PATH: &'static str = "inputs/part1.txt";
const PART_2_INPUT_PATH: &'static str = "inputs/part2.txt";

enum Instruction {
    Increment(String, i64),
    Decrement(String, i64),
}

enum Condition {
    LessThan(String, i64),
    LessThanEqual(String, i64),
    GreaterThan(String, i64),
    GreaterThanEqual(String, i64),
    EqualTo(String, i64),
    NotEqualTo(String, i64),
}

struct ProgramLine {
    pub instruction: Instruction,
    pub condition: Condition,
}

struct Cpu {
    registers: HashMap<String, i64>,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu { registers: HashMap::new() }
    }

    fn get_register_mut(&mut self, name: &str) -> &mut i64 {
        self.registers.entry(name.to_string()).or_insert(0)
    }

    fn get_register_value(&self, name: &str) -> i64 {
        *self.registers.get(name).unwrap_or(&0)
    }

    fn increase_register_value(&mut self, name: &str, amount: i64) -> i64 {
        let register = self.get_register_mut(name);
        *register += amount;
        *register
    }

    fn decrease_register_value(&mut self, name: &str, amount: i64) -> i64 {
        let register = self.get_register_mut(name);
        *register -= amount;
        *register
    }

    fn get_max_register_value(&self) -> i64 {
        *self.registers.values().max().unwrap()
    }

    fn apply_program_line(&mut self, line: &ProgramLine) -> Option<i64> {
        use Condition::*;
        use Instruction::*;

        let condition_met = match line.condition {
            LessThan(ref name, amount) => self.get_register_value(name) < amount,
            LessThanEqual(ref name, amount) => self.get_register_value(name) <= amount,
            GreaterThan(ref name, amount) => self.get_register_value(name) > amount,
            GreaterThanEqual(ref name, amount) => self.get_register_value(name) >= amount,
            EqualTo(ref name, amount) => self.get_register_value(name) == amount,
            NotEqualTo(ref name, amount) => self.get_register_value(name) != amount,
        };

        if condition_met {
            match line.instruction {
                Increment(ref name, amount) => Some(self.increase_register_value(name, amount)),
                Decrement(ref name, amount) => Some(self.decrease_register_value(name, amount)),
            }
        } else {
            None
        }
    }
}

fn parse_instruction(instruction_str: &str) -> Instruction {
    lazy_static! {
        static ref INSTRUCTION_REGEX: Regex = Regex::new("([a-z]+) (inc|dec) (-?[0-9]+)").unwrap();
    }

    let captures = INSTRUCTION_REGEX.captures(instruction_str).unwrap();
    let name = captures[1].to_string();
    let amount: i64 = captures[3].parse().unwrap();

    match &captures[2] {
        "inc" => Instruction::Increment(name, amount),
        "dec" => Instruction::Decrement(name, amount),
        x => panic!("Unrecognised instruction {}", x),
    }
}

fn parse_condition(condition_str: &str) -> Condition {
    lazy_static! {
        static ref CONDITION_REGEX: Regex = Regex::new("([a-z]+) (<|<=|>|>=|==|!=) (-?[0-9]+)").unwrap();
    }

    let captures = CONDITION_REGEX.captures(condition_str).unwrap();
    let name = captures[1].to_string();
    let amount: i64 = captures[3].parse().unwrap();

    match &captures[2] {
        "<" => Condition::LessThan(name, amount),
        "<=" => Condition::LessThanEqual(name, amount),
        ">" => Condition::GreaterThan(name, amount),
        ">=" => Condition::GreaterThanEqual(name, amount),
        "==" => Condition::EqualTo(name, amount),
        "!=" => Condition::NotEqualTo(name, amount),
        x => panic!("Unrecognised condition {}", x),
    }
}

fn parse_program_line(line: &str) -> ProgramLine {
    let parts: Vec<_> = line.split(" if ").collect();
    ProgramLine {
        instruction: parse_instruction(parts[0]),
        condition: parse_condition(parts[1]),
    }
}

fn read_program_lines_from_file(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.unwrap()).collect()
}

fn part1() {
    let mut cpu = Cpu::new();
    let program = read_program_lines_from_file(PART_1_INPUT_PATH);
    for line in program {
        let parsed_line = parse_program_line(&line);
        cpu.apply_program_line(&parsed_line);
    }

    println!("The answer to Part 1 is {}", cpu.get_max_register_value());
}

fn part2() {
    let mut cpu = Cpu::new();
    let program = read_program_lines_from_file(PART_2_INPUT_PATH);
    let mut max_register_value = i64::min_value();

    for line in program {
        let parsed_line = parse_program_line(&line);
        match cpu.apply_program_line(&parsed_line) {
            Some(x) => max_register_value = cmp::max(max_register_value, x),
            None => {},
        }
    }

    println!("The answer to Part 1 is {}", max_register_value);
}

fn main() {
    part1();
    part2();
}
