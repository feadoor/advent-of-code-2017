#![feature(slice_rotate)]

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::from_utf8;

const INPUT_PATH: &'static str = "inputs/input.txt";

#[derive(Copy, Clone)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn parse_move(move_str: &str) -> Move {
    let mut chars = move_str.chars();

    let move_type = chars.next().unwrap();
    let rest: String = chars.collect();

    match move_type {
        's' => Move::Spin(rest.parse().unwrap()),
        'x' => {
            let mut nums = rest.split('/').map(|x| x.parse().unwrap());
            Move::Exchange(nums.next().unwrap(), nums.next().unwrap())
        }
        'p' => {
            let mut names = rest.split('/');
            Move::Partner(names.next().unwrap().chars().nth(0).unwrap(),
                          names.next().unwrap().chars().nth(0).unwrap())
        },
        x => panic!("Unknown move type {}", x),
    }
}

fn get_moves_from_file(path: &str) -> Vec<Move> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    reader.split(b',')
        .map(|mov| parse_move(from_utf8(&mov.unwrap()).unwrap()))
        .collect()
}

fn get_position<T: PartialEq>(element: &T, slice: &[T]) -> Option<usize> {
    slice.iter().position(|x| *x == *element)
}

fn apply_move(programs: &mut Vec<char>, mov: Move) {
    match mov {
        Move::Spin(size) => {
            let len = programs.len();
            programs.rotate(len - size);
        },
        Move::Exchange(ix, jx) => programs.swap(ix, jx),
        Move::Partner(pa, pb) => {
            let ix = get_position(&pa, programs).unwrap();
            let jx = get_position(&pb, programs).unwrap();
            programs.swap(ix, jx);
        }
    }
}

fn repeat_dance(programs: &[char], dance: &[Move], iterations: usize) -> Vec<char> {
    let mut seen = Vec::new();
    let mut current = programs.to_vec();

    for it in 0..iterations {
        match get_position(&current, &seen) {
            Some(ix) => return seen[ix + (iterations % (it - ix))].clone(),
            None => {
                seen.push(current.clone());
                for &mov in dance {
                    apply_move(&mut current, mov);
                }
            }
        }
    }

    current
}

fn part1() {
    let programs = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
                            'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'];

    let dance = get_moves_from_file(INPUT_PATH);
    let final_order = repeat_dance(&programs, &dance, 1);

    println!("The answer to Part 1 is {}", final_order.iter().collect::<String>());
}

fn part2() {
    let programs = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
                        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'];

    let dance = get_moves_from_file(INPUT_PATH);
    let final_order = repeat_dance(&programs, &dance, 1_000_000_000);

    println!("The answer to Part 2 is {}", final_order.iter().collect::<String>());
}

fn main() {
    part1();
    part2();
}
