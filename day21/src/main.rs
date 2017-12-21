use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

const INPUT_PATH: &'static str = "inputs/input.txt";

type Pattern = Vec<Vec<char>>;

type Enhancements = HashMap<Pattern, Pattern>;

type Grid = Vec<Vec<char>>;

fn get_enhancements_from_file(path: &str) -> Enhancements {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut enhancements = HashMap::new();

    for line in reader.lines() {
        let unwrapped = line.unwrap();
        let words: Vec<_> = unwrapped.split_whitespace().collect();
        let rows = |word: &str| word.split('/').map(|r| r.chars().collect()).collect();
        enhancements.insert(rows(words[0]), rows(words[2]));
    }

    enhancements
}

fn starting_grid() -> Grid {
    vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#']
    ]
}

fn rotate(pattern: &Pattern) -> Pattern {
    let size = pattern.len();
    let mut result = vec![vec![' '; size]; size];

    for idx in 0..size {
        for jdx in 0..size {
            result[size - jdx - 1][idx] = pattern[idx][jdx];
        }
    }

    result
}

fn reflect(pattern: &Pattern) -> Pattern {
    let size = pattern.len();
    let mut result = vec![vec![' '; size]; size];

    for idx in 0..size {
        for jdx in 0..size {
            result[idx][size - jdx - 1] = pattern[idx][jdx];
        }
    }

    result
}

fn is_rotation_or_reflection(pattern1: &Pattern, pattern2: &Pattern) -> bool {
    let mut test_pattern = (*pattern1).clone();
    for _reflection in 0..2 {
        for _rotation in 0..4 {
            if test_pattern == *pattern2 {
                return true;
            }
            test_pattern = rotate(&test_pattern);
        }
        test_pattern = reflect(&test_pattern);
    }

    false
}

fn find_matching_enhancement(pattern: &Pattern, enhancements: &Enhancements) -> Pattern {
    enhancements.iter()
        .find(|&(k, _)| is_rotation_or_reflection(pattern, k))
        .unwrap().1
        .clone()
}

fn get_subsquare(grid: &Grid, row: usize, col: usize, size: usize) -> Pattern {
    let mut pattern = vec![vec![' '; size]; size];

    for idx in 0..size {
        for jdx in 0..size {
            pattern[idx][jdx] = grid[row + idx][col + jdx];
        }
    }

    pattern
}

fn set_subsquare(grid: &mut Grid, row: usize, col: usize, pattern: &Pattern) {
    let size = pattern.len();

    for idx in 0..size {
        for jdx in 0..size {
            grid[row + idx][col + jdx] = pattern[idx][jdx];
        }
    }
}

fn apply_enhancement(grid: &Grid, enhancements: &Enhancements) -> Grid {
    let input_size = grid.len();
    let (input_pattern_size, output_pattern_size);

    if input_size % 2 == 0 {
        input_pattern_size = 2; output_pattern_size = 3;
    } else {
        input_pattern_size = 3; output_pattern_size = 4;
    }

    let output_size = (input_size / input_pattern_size) * output_pattern_size;
    let mut output = vec![vec![' '; output_size]; output_size];

    for row in 0..input_size / input_pattern_size {
        for col in 0..input_size / input_pattern_size {
            let pattern = get_subsquare(grid, row * input_pattern_size, col * input_pattern_size, input_pattern_size);
            let enhancement = find_matching_enhancement(&pattern, enhancements);
            set_subsquare(&mut output, row * output_pattern_size, col * output_pattern_size, &enhancement);
        }
    }

    output
}

fn count_set_squares(grid: &Grid) -> usize {
    grid.iter()
        .map(|row| row.iter().filter(|&&c| c == '#').count())
        .sum()
}

fn part1() {
    let enhancements = get_enhancements_from_file(INPUT_PATH);
    let mut grid = starting_grid();
    for _enhancement_round in 0..5 {
        grid = apply_enhancement(&grid, &enhancements);
    }

    println!("The answer to Part 1 is {}", count_set_squares(&grid));
}

fn part2() {
    let enhancements = get_enhancements_from_file(INPUT_PATH);
    let mut grid = starting_grid();
    for _enhancement_round in 0..18 {
        grid = apply_enhancement(&grid, &enhancements);
    }
    println!("The answer to Part 2 is {}", count_set_squares(&grid));
}

fn main() {
    part1();
    part2();
}
