use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap};

const INPUT_PATH: &'static str = "inputs/input.txt";

struct TowerItem {
    weight: u64,
    subtower_weight: u64,
}

impl TowerItem {
    fn new(weight: u64, subtower_weight: u64) -> TowerItem {
        TowerItem {
            weight,
            subtower_weight,
        }
    }

    fn get_weight(&self) -> u64 {
        self.weight
    }

    fn get_subtower_weight(&self) -> u64 {
        self.subtower_weight
    }
}

struct Tower {
    root: String,
    children: HashMap<String, Vec<String>>,
    items: HashMap<String, TowerItem>,
}

impl Tower {
    fn new(weights: HashMap<String, u64>, children: HashMap<String, Vec<String>>) -> Tower {
        let root = weights.keys()
            .filter(|k| !children.values().any(|v| v.contains(k)))
            .next().unwrap().to_string();

        let mut subtower_weights = HashMap::new();
        let mut items_to_visit = vec![root.clone()];
        while !items_to_visit.is_empty() {
            let current_item = items_to_visit.last().unwrap().clone();
            let current_children = children.get(&current_item).unwrap();
            if current_children.iter().all(|c| subtower_weights.contains_key(c)) {
                let subtower_weight: u64 = current_children.iter().map(|c| subtower_weights.get(c).unwrap()).sum();
                subtower_weights.insert(current_item.clone(), subtower_weight + weights.get(&current_item).unwrap());
                items_to_visit.pop();
            } else {
                items_to_visit.extend_from_slice(&current_children);
            }
        }

        let mut items = HashMap::new();
        for k in weights.keys() {
            items.insert(k.to_string(), TowerItem::new(*weights.get(k).unwrap(), *subtower_weights.get(k).unwrap()));
        }

        Tower {
            root,
            children,
            items,
        }
    }

    fn get_root(&self) -> &str {
        &self.root
    }

    fn get_weight(&self, name: &str) -> u64 {
        self.items.get(name).unwrap().get_weight()
    }

    fn get_subtower_weight(&self, name: &str) -> u64 {
        self.items.get(name).unwrap().get_subtower_weight()
    }

    fn get_children(&self, name: &str) -> &[String] {
        self.children.get(name).unwrap()
    }

    fn is_unbalanced(&self, name: &str) -> bool {
        let child_weights: Vec<_> = self.get_children(name).iter()
            .map(|c| self.get_subtower_weight(&c))
            .collect();

        if child_weights.len() < 2 {
            false
        } else {
            child_weights.iter().any(|&c| c != child_weights[0])
        }
    }

    fn has_unbalanced_children(&self, name: &str) -> bool {
        self.get_children(name).iter().any(|c| self.is_unbalanced(c))
    }

    fn get_adjustment_needed_to_balance(&self) -> (&str, u64) {
        let unbalanced_root = self.items.keys()
            .filter(|k| self.is_unbalanced(k))
            .filter(|k| !self.has_unbalanced_children(k))
            .next().unwrap();

        let children = self.get_children(unbalanced_root);

        let bad_child = children.iter().find(|&c| !children.iter().any(|other| c != other && self.get_subtower_weight(other) == self.get_subtower_weight(c))).unwrap();
        let other_child = children.iter().find(|&c| c != bad_child).unwrap();

        (bad_child, self.get_weight(bad_child) + self.get_subtower_weight(other_child) - self.get_subtower_weight(bad_child))
    }
}

fn parse_weight(bracketed_weight: &str) -> u64 {
    let len = bracketed_weight.len();
    bracketed_weight[1..len - 1].parse().unwrap()
}

fn read_tower_from_file(path: &str) -> Tower {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut weights = HashMap::new();
    let mut children = HashMap::new();

    for line in reader.lines().map(|line| line.unwrap()) {
        let mut words = line.split_whitespace();
        let name = words.next().unwrap().to_string();
        let weight = parse_weight(words.next().unwrap());
        let sub_programs: Vec<_> = words
            .map(|word| word.trim_matches(',').to_string())
            .filter(|ref word| word.chars().all(|c| c.is_alphabetic()))
            .collect();

        weights.insert(name.clone(), weight);
        children.insert(name, sub_programs);
    }

    Tower::new(weights, children)
}

fn part1() {
    let tower = read_tower_from_file(INPUT_PATH);
    println!("The answer to Part 1 is {}", tower.get_root());
}

fn part2() {
    let tower = read_tower_from_file(INPUT_PATH);
    let answer = tower.get_adjustment_needed_to_balance().1;
    println!("The answer to Part 2 is {}", answer);
}

fn main() {
    part1();
    part2();
}
