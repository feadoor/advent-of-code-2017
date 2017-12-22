use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

const INPUT_PATH: &'static str = "inputs/input.txt";

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_left(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn rotate_right(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
        }
    }

    fn reverse(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }

    fn get_delta(&self) -> (i64, i64) {
        match * self {
            Direction::Up => (-1, 0),
            Direction::Left => (0, -1),
            Direction::Down => (1, 0),
            Direction::Right => (0, 1),
        }
    }
}

struct Grid1 {
    infected_cells: HashSet<(i64, i64)>,
    position: (i64, i64),
    direction: Direction,
}

impl Grid1 {
    fn new(position: (i64, i64), direction: Direction) -> Grid1 {
        Grid1 {
            infected_cells: HashSet::new(),
            position: position,
            direction: direction,
        }
    }

    fn take_step(&mut self) -> bool {
        let became_infected = !self.infected_cells.remove(&self.position);
        if became_infected {
            self.direction = self.direction.rotate_left();
            self.infected_cells.insert(self.position);
        } else {
            self.direction = self.direction.rotate_right();
        }
        let (dx, dy) = self.direction.get_delta();
        self.position = (self.position.0 + dx, self.position.1 + dy);

        became_infected
    }
}

struct Grid2 {
    infected_cells: HashSet<(i64, i64)>,
    weakened_cells: HashSet<(i64, i64)>,
    flagged_cells: HashSet<(i64, i64)>,
    position: (i64, i64),
    direction: Direction,
}

impl Grid2 {
    fn new(position: (i64, i64), direction: Direction) -> Grid2 {
        Grid2 {
            infected_cells: HashSet::new(),
            weakened_cells: HashSet::new(),
            flagged_cells: HashSet::new(),
            position: position,
            direction: direction,
        }
    }

    fn take_step(&mut self) -> bool {
        let mut became_infected = false;

        if self.weakened_cells.remove(&self.position) {
            self.infected_cells.insert(self.position);
            became_infected = true;
        } else if self.infected_cells.remove(&self.position) {
            self.direction = self.direction.rotate_right();
            self.flagged_cells.insert(self.position);
        } else if self.flagged_cells.remove(&self.position) {
            self.direction = self.direction.reverse();
        } else {
            self.weakened_cells.insert(self.position);
            self.direction = self.direction.rotate_left();
        }

        let (dx, dy) = self.direction.get_delta();
        self.position = (self.position.0 + dx, self.position.1 + dy);

        became_infected
    }
}

fn get_grid1_from_file(path: &str) -> Grid1 {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut infected_cells = HashSet::new();

    let mut row_count = 0;
    let mut col_count = 0;

    for (idx, line) in reader.lines().enumerate() {
        let unwrapped_line = line.unwrap();
        for (jdx, cell) in unwrapped_line.chars().enumerate() {
            if cell == '#' {
                infected_cells.insert((idx as i64, jdx as i64));
            }
            col_count = jdx as i64;
        }
        row_count = idx as i64;
    }

    let mut grid = Grid1::new(((row_count + 1) / 2, (col_count + 1) / 2), Direction::Up);
    grid.infected_cells = infected_cells;
    grid
}

fn get_grid2_from_file(path: &str) -> Grid2 {
    let grid1 = get_grid1_from_file(path);
    let mut grid2 = Grid2::new(grid1.position, grid1.direction);
    grid2.infected_cells = grid1.infected_cells;
    grid2
}

fn part1() {
    let mut grid: Grid1 = get_grid1_from_file(INPUT_PATH);
    let mut infections = 0;
    for _ in 0..10_000 {
        if grid.take_step() {
            infections += 1;
        }
    }

    println!("The answer to Part 1 is {}", infections);
}

fn part2() {
    let mut grid: Grid2 = get_grid2_from_file(INPUT_PATH);
    let mut infections = 0;
    for _ in 0..10_000_000 {
        if grid.take_step() {
            infections += 1;
        }
    }

    println!("The answer to Part 2 is {}", infections);
}

fn main() {
    part1();
    part2();
}
