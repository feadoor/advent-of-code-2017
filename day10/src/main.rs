#![feature(slice_rotate)]
#![feature(inclusive_range_syntax)]

use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use std::str::from_utf8;

const INPUT_PATH: &'static str = "inputs/input.txt";

fn get_lengths_from_file(path: &str) -> Vec<u8> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    reader.split(b',')
        .map(|x| x.unwrap())
        .map(|x| from_utf8(&x).unwrap().to_string())
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_bytes_from_file(path: &str) -> Vec<u8> {
    let file = File::open(path).expect("Unable to open file");
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).expect("Unable to read file");
    buf
}

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

fn as_hex(byte: u8) -> String {
    static CHARS: &'static [&str] = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"];
    let (hi, lo) = (byte / 16, byte % 16);
    CHARS[hi as usize].to_string() + CHARS[lo as usize]
}

fn get_hash(bytes: &[u8]) -> String {
    let mut twisted_knot: Vec<_> = (0..=255).collect();
    let mut padded_lengths = bytes.to_vec();
    padded_lengths.extend_from_slice(&[17, 31, 73, 47, 23]);

    apply_hashing_rounds(&mut twisted_knot, &padded_lengths, 64);
    twisted_knot.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, curr| acc ^ curr))
        .fold(String::new(), |acc, curr| acc + &as_hex(curr))
}

fn part1() {
    let mut knot: Vec<_> = (0..=255).collect();
    let lengths = get_lengths_from_file(INPUT_PATH);
    apply_hashing_rounds(&mut knot, &lengths, 1);
    println!("The answer to Part 1 is {}", knot[0] as usize * knot[1] as usize);
}

fn part2() {
    let bytes = get_bytes_from_file(INPUT_PATH);
    println!("The answer to Part 2 is {}", get_hash(&bytes));
}

fn main() {
    part1();
    part2();
}
