use std::fs::File;
use std::io::{BufReader, BufRead};

const INPUT_PATH: &'static str = "inputs/input.txt";

fn read_spreadsheet_from_file(path: &str) -> Vec<Vec<u64>> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    reader.lines()
        .map(|line| line.unwrap())
        .map(|line| line.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect())
        .collect()
}

fn checksum(spreadsheet: &[Vec<u64>]) -> u64 {
    spreadsheet.iter().map(|row| row.iter().max().unwrap() - row.iter().min().unwrap()).sum()
}

fn sum_of_even_divisions(spreadsheet: &[Vec<u64>]) -> u64 {
    let mut sum = 0;

    for row in spreadsheet {
        'outer: for num in row {
            for other in row {
                if other != num && other % num == 0 {
                    sum += other / num;
                    break 'outer;
                }
            }
        }
    }

    sum
}

fn part1() {
    let spreadsheet = read_spreadsheet_from_file(INPUT_PATH);
    println!("The answer to Part 1 is {}", checksum(&spreadsheet));
}

fn part2() {
    let spreadsheet = read_spreadsheet_from_file(INPUT_PATH);
    println!("The answer to Part 2 is {}", sum_of_even_divisions(&spreadsheet));
}

fn main() {
    part1();
    part2();
}