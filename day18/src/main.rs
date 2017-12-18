use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, VecDeque};

const INPUT_PATH: &'static str = "inputs/input.txt";

enum Value {
    Register(String),
    Direct(i64),
}

enum Instruction {
    Snd(Value),
    Set(String, Value),
    Add(String, Value),
    Mul(String, Value),
    Mod(String, Value),
    Rcv(String),
    Jgz(Value, Value),
}

struct SingleProgramRunner<'a> {
    program: &'a [Instruction],
    current_instruction: i64,
    registers: HashMap<String, i64>,
    last_frequency: Option<i64>,
}

impl<'a> SingleProgramRunner<'a> {
    fn new(program: &'a [Instruction]) -> SingleProgramRunner<'a> {
        SingleProgramRunner {
            program: program,
            current_instruction: 0,
            registers: HashMap::new(),
            last_frequency: None,
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

    fn step(&mut self) -> Option<i64> {
        use Instruction::*;

        let mut ret = None;
        if self.in_bounds() {
            let old_instruction = self.current_instruction;
            match self.program[self.current_instruction as usize] {
                Snd(ref val) => self.last_frequency = Some(self.get_value(val)),
                Set(ref reg, ref val) => *self.get_register_mut(reg) = self.get_value(val),
                Add(ref reg, ref val) => *self.get_register_mut(reg) += self.get_value(val),
                Mul(ref reg, ref val) => *self.get_register_mut(reg) *= self.get_value(val),
                Mod(ref reg, ref val) => *self.get_register_mut(reg) %= self.get_value(val),
                Rcv(ref reg) => if *self.get_register_mut(reg) != 0 { ret = self.last_frequency; },
                Jgz(ref val1, ref val2) => if self.get_value(val1) > 0 { self.current_instruction += self.get_value(val2); }
            }
            if self.current_instruction == old_instruction {
                self.current_instruction += 1;
            }
        }
        ret
    }
}

struct Duettist<'a> {
    program: &'a [Instruction],
    current_instruction: i64,
    registers: HashMap<String, i64>,
    value_queue: VecDeque<i64>,
    snd_count: usize,
}

impl<'a> Duettist<'a> {
    fn new(pid: i64, program: &'a [Instruction]) -> Duettist {
        let mut registers = HashMap::new();
        registers.insert("p".to_string(), pid);
        Duettist {
            program: program,
            current_instruction: 0,
            registers: registers,
            value_queue: VecDeque::new(),
            snd_count: 0,
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

    fn rcv_value(&mut self) -> Option<i64> {
        self.value_queue.pop_front()
    }

    fn in_bounds(&self) -> bool {
        self.current_instruction >= 0 &&
        self.current_instruction < self.program.len() as i64
    }

    fn step(&mut self) -> Result<Option<i64>, ()> {
        use Instruction::*;

        let mut sent_value = None;
        if self.in_bounds() {
            let old_instruction = self.current_instruction;
            match self.program[self.current_instruction as usize] {
                Snd(ref val) => { sent_value = Some(self.get_value(val)); self.snd_count += 1; },
                Set(ref reg, ref val) => *self.get_register_mut(reg) = self.get_value(val),
                Add(ref reg, ref val) => *self.get_register_mut(reg) += self.get_value(val),
                Mul(ref reg, ref val) => *self.get_register_mut(reg) *= self.get_value(val),
                Mod(ref reg, ref val) => *self.get_register_mut(reg) %= self.get_value(val),
                Rcv(ref reg) => if let Some(val) = self.rcv_value() { *self.get_register_mut(reg) = val; } else { return Err(()); },
                Jgz(ref val1, ref val2) => if self.get_value(val1) > 0 { self.current_instruction += self.get_value(val2); }
            }
            if self.current_instruction == old_instruction {
                self.current_instruction += 1;
            }
            Ok(sent_value)
        } else {
            Err(())
        }
    }
}

struct DuetProgramRunner<'a> {
    duettists: (Duettist<'a>, Duettist<'a>),
}

impl<'a> DuetProgramRunner<'a> {
    fn new(program: &'a [Instruction]) -> DuetProgramRunner<'a> {
        DuetProgramRunner {
            duettists: (Duettist::new(0, program), Duettist::new(1, program)),
        }
    }

    fn step(&mut self) -> Result<(), ()> {
        let result = (self.duettists.0.step(), self.duettists.1.step());
        if result.0.is_err() && result.1.is_err() {
            Err(())
        } else {
            if let Ok(Some(x)) = result.0 { self.duettists.1.value_queue.push_back(x) }
            if let Ok(Some(x)) = result.1 { self.duettists.0.value_queue.push_back(x) }
            Ok(())
        }
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
        "snd" => Instruction::Snd(next_value!()),
        "set" => Instruction::Set(next_register!(), next_value!()),
        "add" => Instruction::Add(next_register!(), next_value!()),
        "mul" => Instruction::Mul(next_register!(), next_value!()),
        "mod" => Instruction::Mod(next_register!(), next_value!()),
        "rcv" => Instruction::Rcv(next_register!()),
        "jgz" => Instruction::Jgz(next_value!(), next_value!()),
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

fn part1() {
    let program = get_program_from_file(INPUT_PATH);
    let mut runner = SingleProgramRunner::new(&program);

    loop {
        if let Some(freq) = runner.step() {
            println!("The answer to Part 1 is {}", freq);
            break;
        }
    }
}

fn part2() {
    let program = get_program_from_file(INPUT_PATH);
    let mut runner = DuetProgramRunner::new(&program);
    while let Ok(()) = runner.step() { }
    println!("The answer to Part 2 is {}", runner.duettists.1.snd_count);
}

fn main() {
    part1();
    part2();
}
