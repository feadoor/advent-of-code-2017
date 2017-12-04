use std::fs::File;
use std::io::prelude::*;

const PART_1_INPUT_PATH: &'static str = "inputs/part1.txt";
const PART_2_INPUT_PATH: &'static str = "inputs/part2.txt";

fn sum_of_digits_equal_to_partner<F: Fn(usize) -> usize>(input: &[u8], partner_index: F) -> u64 {
    input.iter().enumerate()
        .filter(|&(ix, byte)| *byte == input[partner_index(ix)])
        .map(|(_, byte)| (byte - b'0') as u64)
        .sum()
}

fn sum_of_digits_equal_to_immediate_neighbour(input: &[u8]) -> u64 {
    sum_of_digits_equal_to_partner(input, |ix| if ix + 1 == input.len() { 0 } else { ix + 1})
}

fn sum_of_digits_equal_to_halfway_partner(input: &[u8]) -> u64 {
    let half_length = input.len() / 2;
    sum_of_digits_equal_to_partner(
        input,
        |ix| if ix + half_length >= input.len() { ix - half_length} else { ix + half_length }
    )
}

fn read_file_to_string(path: &str) -> String {
    let mut result = String::new();
    let mut file = File::open(path).expect("Unable to open file");
    file.read_to_string(&mut result).expect("Unable to read file");
    result
}

fn part1() {
    let captcha = read_file_to_string(PART_1_INPUT_PATH);
    let answer = sum_of_digits_equal_to_immediate_neighbour(&captcha.into_bytes());
    println!("The answer to Part 1 is {}", answer);
}

fn part2() {
    let captcha = read_file_to_string(PART_2_INPUT_PATH);
    let answer = sum_of_digits_equal_to_halfway_partner(&captcha.into_bytes());
    println!("The answer to Part 2 is {}", answer);
}

fn main() {
    part1();
    part2();
}