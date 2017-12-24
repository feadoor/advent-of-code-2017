mod dfs;

use std::fs::File;
use std::io::{BufReader, BufRead};
use dfs::DepthFirstTree;

const INPUT_PATH: &'static str = "inputs/input.txt";

type Port = [u64; 2];

fn get_ports_from_file(path: &str) -> Vec<Port> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    reader.lines()
        .map(|line| line.unwrap())
        .map(|line| line.split('/').map(|pins| pins.parse().unwrap()).collect())
        .map(|port: Vec<u64>| [port[0], port[1]])
        .collect()
}

fn port_side_with_pins(port: &Port, pins: u64) -> Option<usize> {
    if port[0] == pins {
        Some(0)
    } else if port[1] == pins {
        Some(1)
    } else {
        None
    }
}

struct BridgeTree {
    ports: Vec<Port>,
    used_ports: Vec<bool>,
    current_ports: Vec<Port>,
    used_sides: Vec<usize>,
}

impl BridgeTree {
    fn new(ports: Vec<Port>) -> BridgeTree {
        let size = ports.len();

        BridgeTree {
            ports: ports,
            used_ports: vec![false; size],
            current_ports: vec![[0, 0]],
            used_sides: vec![0],
        }
    }
}

struct BridgeTreeStep {
    next_idx: usize,
    next_port: Port,
    used_side: usize,
}

impl BridgeTreeStep {
    fn new(idx: usize, port: Port, side: usize) -> BridgeTreeStep {
        BridgeTreeStep {
            next_idx: idx,
            next_port: port,
            used_side: side,
        }
    }
}

impl DepthFirstTree for BridgeTree {
    type Step = BridgeTreeStep;
    type Output = Vec<Port>;

    fn next_steps(&mut self) -> Vec<Self::Step> {
        if self.current_ports.len() == 0 {
            self.ports.iter().enumerate()
                .filter(|&(idx, _)| !self.used_ports[idx])
                .flat_map(|(idx, port)| vec![(idx, port.clone(), 0), (idx, port.clone(), 1)].into_iter())
                .map(|(idx, port, side)| BridgeTreeStep::new(idx, port, side))
                .collect()
        } else {
            let side_to_match = self.current_ports.last().unwrap()[*self.used_sides.last().unwrap()];
            self.ports.iter().enumerate()
                .filter(|&(idx, _)| !self.used_ports[idx])
                .filter(|&(_, port)| port_side_with_pins(&port, side_to_match).is_some())
                .map(|(idx, port)| BridgeTreeStep::new(idx, *port, 1 - port_side_with_pins(&port, side_to_match).unwrap()))
                .collect()
        }
    }

    fn apply_step(&mut self, step: &Self::Step) {
        self.used_ports[step.next_idx] = true;
        self.current_ports.push(step.next_port);
        self.used_sides.push(step.used_side);
    }

    fn revert_step(&mut self, step: &Self::Step) {
        self.used_ports[step.next_idx] = false;
        self.current_ports.pop();
        self.used_sides.pop();
    }

    fn output(&mut self) -> Option<Self::Output> {
        if self.next_steps().len() == 0 {
            Some(self.current_ports.clone())
        } else {
            None
        }
    }
}

fn part1() {
    let ports = get_ports_from_file(INPUT_PATH);
    let mut search_tree = BridgeTree::new(ports);
    let maximal_weight: u64 = search_tree.iter()
        .map(|bridge| bridge.iter().map(|p| p[0] + p[1]).sum())
        .max().unwrap();

    println!("The answer to Part 1 is {}", maximal_weight);
}

fn part2() {
    let ports = get_ports_from_file(INPUT_PATH);
    let mut search_tree = BridgeTree::new(ports);
    let weight_of_longest_bridge: u64 = search_tree.iter()
        .map(|bridge| (bridge.len(), bridge.iter().map(|p| p[0] + p[1]).sum()))
        .max().unwrap().1;

    println!("The answer to Part 2 is {}", weight_of_longest_bridge);
}

fn main() {
    part1();
    part2();
}
