use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_PATH: &'static str = "inputs/input.txt";

fn read_maze_from_file(path: &str) -> Vec<i64> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    reader.lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

fn steps_to_exit_with_rule<F: Fn(i64) -> i64>(maze: &mut [i64], rule: F) -> u64 {
    let mut steps = 0;
    let mut current_idx: i64 = 0;

    loop {
        let idx = current_idx as usize;
        current_idx += maze[idx];
        maze[idx] = rule(maze[idx]);
        steps += 1;

        if current_idx < 0 || current_idx >= maze.len() as i64 {
            break;
        }
    }

    steps
}



fn part1() {
    let mut maze = read_maze_from_file(INPUT_PATH);
    let answer = steps_to_exit_with_rule(&mut maze, |offset| offset + 1);
    println!("The answer to Part 1 is {}", answer);
}

fn part2() {
    let mut maze = read_maze_from_file(INPUT_PATH);
    let answer = steps_to_exit_with_rule(
        &mut maze,
        |offset| if offset >= 3 { offset - 1 } else { offset + 1 }
    );
    println!("The answer to Part 2 is {}", answer);
}

fn main() {
    part1();
    part2();
}
