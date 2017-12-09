use std::fs::File;
use std::io::prelude::*;

const PART_1_INPUT_PATH: &'static str = "inputs/part1.txt";
const PART_2_INPUT_PATH: &'static str = "inputs/part2.txt";

struct StringStats {
    pub score: u64,
    pub garbage_chars: usize,
}

impl StringStats {
    fn new() -> StringStats {
        StringStats {
            score: 0,
            garbage_chars: 0,
        }
    }
}

fn read_file_to_string(path: &str) -> String {
    let mut result = String::new();
    let mut file = File::open(path).expect("Unable to open file");
    file.read_to_string(&mut result).expect("Unable to read file");
    result
}

fn stats(input: &str) -> StringStats {
    let mut depth = 0;
    let mut garbage = false;
    let mut ignore = false;

    let mut stats = StringStats::new();

    for c in input.chars() {
        if ignore {
            ignore = false;
        } else if garbage {
            match c {
                '>' => garbage = false,
                '!' => ignore = true,
                _ => stats.garbage_chars += 1,
            }
        } else {
            match c {
                '{' => {
                    depth += 1;
                    stats.score += depth;
                },
                '}' => depth -= 1,
                '<' => garbage = true,
                _ => {},
            }
        }
    }

    stats
}

fn part1() {
    let input = read_file_to_string(PART_1_INPUT_PATH);
    println!("The answer to Part 1 is {}", stats(&input).score);
}

fn part2() {
    let input = read_file_to_string(PART_2_INPUT_PATH);
    println!("The answer to Part 2 is {}", stats(&input).garbage_chars);
}

fn main() {
    part1();
    part2();
}
