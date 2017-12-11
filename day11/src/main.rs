use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::from_utf8;
use std::cmp::max;

const INPUT_PATH: &'static str = "inputs/input.txt";

#[derive(Copy, Clone)]
enum Step {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

fn get_steps_from_file(path: &str) -> Vec<Step> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    reader.split(b',').map(|d| d.unwrap()).map(|d|
        match from_utf8(&d).unwrap() {
            "n" => Step::North,
            "ne" => Step::NorthEast,
            "se" => Step::SouthEast,
            "s" => Step::South,
            "sw" => Step::SouthWest,
            "nw" => Step::NorthWest,
            other => panic!("Unknown direction {}", other),
        }
    ).collect()
}

fn take_step(x: i64, y: i64, step: Step) -> (i64, i64) {
    match step {
        Step::North => (x + 1, y),
        Step::NorthEast => (x + 1, y + 1),
        Step::SouthEast => (x, y + 1),
        Step::South => (x - 1, y),
        Step::SouthWest => (x - 1, y - 1),
        Step::NorthWest => (x, y - 1),
    }
}

fn distance_from_centre(x: i64, y: i64) -> i64 {
    max((x - y).abs(), max(x.abs(), y.abs()))
}

fn part1() {
    let (mut x, mut y) = (0, 0);

    for step in get_steps_from_file(INPUT_PATH) {
        let (new_x, new_y) = take_step(x, y, step);
        x = new_x; y = new_y;
    }

    println!("The answer to Part 1 is {}", distance_from_centre(x, y));
}

fn part2() {
    let (mut x, mut y) = (0, 0);
    let mut furthest = 0;

    for step in get_steps_from_file(INPUT_PATH) {
        let (new_x, new_y) = take_step(x, y, step);
        x = new_x; y = new_y;
        furthest = max(furthest, distance_from_centre(x, y));
    }

    println!("The answer to Part 2 is {}", furthest);
}

fn main() {
    part1();
    part2();
}
