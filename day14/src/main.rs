#![feature(slice_rotate)]
#![feature(inclusive_range_syntax)]

const INPUT: &'static str = "ugkiagan";

fn reverse_section(slice: &mut [u8], start: usize, end: usize) {
    if start < end {
        slice[start..end + 1].reverse();
    } else if start > end {
        let reverse_len = (slice.len() - start) + end;
        slice.rotate(start);
        slice[0..reverse_len + 1].reverse();
        slice.rotate(reverse_len - end);
    }
}

fn apply_hashing_rounds(slice: &mut [u8], lengths: &[u8], rounds: usize) {
    let mut skip_size = 0;
    let mut start: usize = 0;
    let knot_length = slice.len();

    for _ in 0..rounds {
        for &length in lengths {
            if length != 0 {
                let end = (start + length as usize - 1) % knot_length;
                reverse_section(slice, start, end);
            }
            start = (start + length as usize + skip_size) % knot_length;
            skip_size += 1;
        }
    }
}

fn get_hash(bytes: &[u8]) -> String {
    let mut twisted_knot: Vec<_> = (0..=255).collect();
    let mut padded_lengths = bytes.to_vec();
    padded_lengths.extend_from_slice(&[17, 31, 73, 47, 23]);

    apply_hashing_rounds(&mut twisted_knot, &padded_lengths, 64);
    twisted_knot.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, curr| acc ^ curr))
        .fold(String::new(), |acc, curr| acc + &format!("{:8b}", curr))
}

fn get_defrag_grid(input: &str) -> Vec<Vec<bool>> {
    let row_input = |row| format!("{}-{}", input, row);
    (0..128).map(
        |row| get_hash(row_input(row).as_bytes()).chars().map(|c| c == '1').collect()
    ).collect()
}

fn get_used_squares_in_grid(input: &str) -> usize {
    let grid = get_defrag_grid(input);
    grid.iter().map(|row| row.iter().filter(|&&b| b).count()).sum()
}

fn count_connected_components(grid: &[Vec<bool>]) -> usize {
    let mut components = 0;

    let size = grid.len();
    for row in grid { assert_eq!(row.len(), size); }
    let mut marked_grid = vec![vec![false; size]; size];

    let next_unmarked_entry = |g: &[Vec<bool>], mg: &[Vec<bool>]| {
        for row in 0..size {
            for col in 0..size {
                if g[row][col] && !mg[row][col] {
                    return Some((row, col));
                }
            }
        }
        None
    };

    let neighbours_of = |x, y| {
        vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .iter()
            .filter(|&&(x, y)| x < size && y < size)
            .map(|a| a.clone())
            .collect::<Vec<_>>()
    };

    while let Some((x, y)) = next_unmarked_entry(grid, &marked_grid) {
        components += 1;
        let mut stack = vec![(x, y)];
        while let Some((next_x, next_y)) = stack.pop() {
            marked_grid[next_x][next_y] = true;
            for (neighbour_x, neighbour_y) in neighbours_of(next_x, next_y) {
                if grid[neighbour_x][neighbour_y] && !marked_grid[neighbour_x][neighbour_y] {
                    stack.push((neighbour_x, neighbour_y));
                }
            }
        }
    }

    components
}

fn part1() {
    println!("The answer to Part 1 is {}", get_used_squares_in_grid(INPUT));
}

fn part2() {
    let grid = get_defrag_grid(INPUT);
    println!("The answer to Part w is {}", count_connected_components(&grid));
}

fn main() {
    part1();
    part2();
}
