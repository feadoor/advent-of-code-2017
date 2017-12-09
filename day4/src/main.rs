use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

const INPUT_PATH: &'static str = "inputs/input.txt";

fn read_words_from_file(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.unwrap()).collect()
}

fn contains_no_repeats_with_map<F: Fn(&str) -> String>(passphrase: &str, mapper: F) -> bool {
    let mut seen_words: HashSet<String> = HashSet::new();
    for word in passphrase.split_whitespace().map(|word| mapper(word)) {
        if seen_words.contains(&word) {
            return false;
        }
        seen_words.insert(word);
    }
    true
}

fn sort_chars_of_string(word: &str) -> String {
    let mut chars: Vec<u8> = word.bytes().collect();
    chars.sort();
    String::from_utf8(chars).unwrap()
}

fn is_valid(passphrase: &str) -> bool {
    contains_no_repeats_with_map(passphrase, |word| word.to_string())
}

fn is_valid_with_anagrams(passphrase: &str) -> bool {
    contains_no_repeats_with_map(passphrase, |word| sort_chars_of_string(word))
}

fn part1() {
    let passphrases = read_words_from_file(INPUT_PATH);
    let answer = passphrases.iter().filter(|pass| is_valid(pass)).count();

    println!("The answer to Part 1 is {}", answer);
}

fn part2() {
    let passphrases = read_words_from_file(INPUT_PATH);
    let answer = passphrases.iter().filter(|pass| is_valid_with_anagrams(pass)).count();

    println!("The answer to Part 2 is {}", answer);
}

fn main() {
    part1();
    part2();
}