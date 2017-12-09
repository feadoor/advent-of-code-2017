use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

const INPUT_PATH: &'static str = "inputs/input.txt";

fn read_vector_from_file(path: &str) -> Vec<u64> {
    let mut file = File::open(path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");

    contents.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect()
}

fn index_of_max(vector: &[u64]) -> usize {
    let (mut best_value, mut best_idx) = (0, 0);
    for (idx, &value) in vector.iter().enumerate() {
        if value > best_value {
            best_value = value;
            best_idx = idx;
        }
    }

    best_idx
}

fn redistribute_blocks(memory_bank: &mut [u64]) {
    let max_idx = index_of_max(memory_bank);
    let blocks = memory_bank[max_idx];
    memory_bank[max_idx] = 0;
    let mut current_idx = max_idx;
    for _ in 0..blocks {
        current_idx = if current_idx + 1 == memory_bank.len() { 0 } else { current_idx + 1 };
        memory_bank[current_idx] += 1;
    }
}

fn period_of_redistribution(memory_bank: &[u64]) -> (usize, usize) {
    let mut worker = memory_bank.to_vec();
    let mut trials = 0;
    let mut seen = HashMap::new();

    loop {
        if seen.contains_key(&worker) {
            break;
        } else {
            seen.insert(worker.clone(), trials);
            redistribute_blocks(&mut worker);
            trials += 1;
        }
    }

    let period = seen.len() - seen.get(&worker).unwrap();
    let tail = trials - period;
    (period, tail)
}

fn part1() {
    let memory_bank = read_vector_from_file(INPUT_PATH);
    let (period, tail) = period_of_redistribution(&memory_bank);
    println!("The answer to Part 1 is {}", tail + period);
}

fn part2() {
    let memory_bank = read_vector_from_file(INPUT_PATH);
    let (period, _) = period_of_redistribution(&memory_bank);
    println!("The answer to Part 2 is {}", period);
}

fn main() {
    part1();
    part2();
}
