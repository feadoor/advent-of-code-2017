use std::fs::File;
use std::io::{BufReader, BufRead};

const INPUT_PATH: &'static str = "inputs/input.txt";

fn get_routing_diagram_from_file(path: &str) -> Vec<Vec<char>> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    reader.lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect())
        .collect()
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn go_in_direction(x: usize, y: usize, direction: Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (x - 1, y),
        Direction::Down => (x + 1, y),
        Direction::Left => (x, y - 1),
        Direction::Right => (x, y + 1),
    }
}

fn is_in_path(x: usize, y: usize, routes: &[Vec<char>]) -> bool {
    x < routes.len() && y < routes[x].len() && routes[x][y] != ' '
}

fn opposite(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}

fn follow_routing_diagram(routes: &[Vec<char>]) -> (Vec<char>, usize) {
    let (mut x, mut y) = (0, routes[0].iter().position(|&x| x != ' ').unwrap());
    let mut direction = Direction::Down;
    let mut letters = Vec::new();
    let mut steps = 1;

    const DIRECTIONS: &'static [Direction] =
        &[Direction::Up, Direction::Down, Direction::Left, Direction::Right];

    loop {
        let (new_x, new_y) = go_in_direction(x, y, direction);
        if is_in_path(new_x, new_y, routes) {
            x = new_x; y = new_y; steps += 1;
            if routes[x][y] == '+' {
                for &new_direction in DIRECTIONS {
                    let (next_x, next_y) = go_in_direction(x, y, new_direction);
                    if new_direction != opposite(direction) && is_in_path(next_x, next_y, routes) {
                        direction = new_direction;
                        break;
                    }
                }
            } else if routes[x][y].is_alphabetic() {
                letters.push(routes[x][y]);
            }
        } else {
            break;
        }
    }

    (letters, steps)
}

fn part1() {
    let routes = get_routing_diagram_from_file(INPUT_PATH);
    let (letters, _) = follow_routing_diagram(&routes);
    println!("The answer to Part 1 is {}", letters.iter().collect::<String>());
}

fn part2() {
    let routes = get_routing_diagram_from_file(INPUT_PATH);
    let (_, steps) = follow_routing_diagram(&routes);
    println!("The answer to Part 2 is {}", steps);
}

fn main() {
    part1();
    part2();
}
