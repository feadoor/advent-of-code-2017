use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

const INPUT_PATH: &'static str = "inputs/input.txt";

fn get_scanners_from_file(path: &str) -> HashMap<u64, u64> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut scanners = HashMap::new();

    for line in reader.lines().map(|line| line.unwrap()) {
        let mut words = line.split_whitespace();
        let depth = words.next().unwrap().trim_matches(':').parse().unwrap();
        let range = words.next().unwrap().parse().unwrap();
        scanners.insert(depth, range);
    }

    scanners
}

fn get_severity(scanners: &HashMap<u64, u64>) -> u64 {
    scanners.iter()
        .filter(|&(&depth, &range)| depth % (2 * (range - 1)) == 0)
        .map(|(&depth, &range)| depth * range)
        .sum()
}

fn get_smallest_safe_delay(scanners: &HashMap<u64, u64>) -> u64 {
    let is_safe = |delay: u64| !scanners.iter()
        .any(|(&depth, &range)| (depth + delay) % (2 * (range - 1)) == 0);
    (0..).find(|&delay| is_safe(delay)).unwrap()
}

fn part1() {
    let scanners = get_scanners_from_file(INPUT_PATH);
    println!("The answer to Part 1 is {}", get_severity(&scanners));
}

fn part2() {
    let scanners = get_scanners_from_file(INPUT_PATH);
    println!("The answer to Part 2 is {}", get_smallest_safe_delay(&scanners));
}

fn main() {
    part1();
    part2();
}