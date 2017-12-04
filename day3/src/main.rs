use std::collections::HashMap;

const INPUT: u64 = 289326;

fn integer_sqrt(n: u64) -> u64 {
    let mut sqrt = (n as f64).sqrt().floor() as u64;
    while sqrt > 0 && sqrt.saturating_mul(sqrt) > n {
        sqrt -= 1;
    }
    while (sqrt + 1).saturating_mul(sqrt + 1) <= n {
        sqrt += 1
    }
    sqrt
}

fn position_in_spiral(num: u64) -> (i64, i64) {
    let layer = ((integer_sqrt(num - 1) + 1) / 2) as i64;
    let num = num as i64;

    let bottom_right: i64 = (2 * layer + 1) * (2 * layer + 1);
    let bottom_left: i64 = bottom_right - 2 * layer;
    let top_left: i64 = bottom_left - 2 * layer;
    let top_right: i64 = top_left - 2 * layer;

    if num <= top_right {
        (layer, layer - (top_right - num))
    } else if num <= top_left {
        (-layer + (top_left - num), layer)
    } else if num <= bottom_left {
        (-layer, -layer + (bottom_left - num))
    } else {
        (layer - (bottom_right - num), -layer)
    }
}

fn neighbours(pos: (i64, i64)) -> Vec<(i64, i64)> {
    let (x, y) = pos;
    vec![
        (x + 1, y),
        (x + 1, y + 1),
        (x, y + 1),
        (x - 1, y + 1),
        (x - 1, y),
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
    ]
}

fn manhattan_distance(num: u64) -> i64 {
    let position = position_in_spiral(num);
    position.0.abs() + position.1.abs()
}

fn part1() {
    println!("The answer to Part 1 is {}", manhattan_distance(INPUT));
}

fn part2() {
    let mut values: HashMap<(i64, i64), u64> = HashMap::new();
    values.insert((0, 0), 1);

    let mut current_idx = 2;
    let mut current_value: u64;

    loop {
        let current_pos = position_in_spiral(current_idx);
        current_value = neighbours(current_pos).iter()
            .map(|&neighbour| *values.entry(neighbour).or_insert(0))
            .sum();

        if current_value > INPUT {
            break;
        } else {
            values.insert(current_pos, current_value);
            current_idx += 1;
        }
    }

    println!("The answer to Part 2 is {}", current_value);
}

fn main() {
    part1();
    part2();
}