use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

const INPUT_PATH: &'static str = "inputs/input.txt";

fn get_adjacencies_from_file(path: &str) -> Vec<Vec<usize>> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut adjacencies = Vec::new();

    for line in reader.lines().map(|line| line.unwrap()) {
        let mut words = line.split_whitespace();
        let neighbours = words.skip(1)
            .map(|word| word.trim_matches(',').parse())
            .filter_map(|num| if let Ok(x) = num { Some(x) } else { None })
            .collect();
        adjacencies.push(neighbours);
    }

    adjacencies
}

fn get_connected_components(adjacencies: &[Vec<usize>]) -> Vec<HashSet<usize>> {
    let mut components = Vec::new();
    let mut seen: Vec<_> = vec![false; adjacencies.len()];

    for program in 0..adjacencies.len() {
        if !seen[program] {

            let mut queue = vec![program];
            let mut neighbours = HashSet::new();

            while let Some(neighbour) = queue.pop() {
                seen[neighbour] = true;
                neighbours.insert(neighbour);
                for &other in adjacencies[neighbour].iter() {
                    if !seen[other] {
                        queue.push(other)
                    }
                }
            }

            components.push(neighbours);
        }
    }

    components
}

fn part1() {
    let adjacencies = get_adjacencies_from_file(INPUT_PATH);
    let components = get_connected_components(&adjacencies);
    println!("The answer to Part 1 is {}", components[0].len());
}

fn part2() {
    let adjacencies = get_adjacencies_from_file(INPUT_PATH);
    let components = get_connected_components(&adjacencies);
    println!("The answer to Part 2 is {}", components.len());
}

fn main() {
    part1();
    part2();
}
