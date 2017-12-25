use std::collections::HashMap;

struct State {
    if_false: Rule,
    if_true: Rule,
}

struct Rule {
    write_value: bool,
    move_offset: isize,
    next_state: usize,
}

struct TuringMachine {
    tape: HashMap<isize, bool>,
    cursor: isize,
    state: usize,
    states: Vec<State>,
}

impl TuringMachine {
    fn new(states: Vec<State>, starting_state: usize) -> TuringMachine {
        TuringMachine {
            tape: HashMap::new(),
            cursor: 0,
            state: starting_state,
            states: states,
        }
    }

    fn run_step(&mut self) {
        let slot = self.tape.entry(self.cursor).or_insert(false);
        let rule = match *slot {
            false => &self.states[self.state].if_false,
            true => &self.states[self.state].if_true,
        };

        *slot = rule.write_value;
        self.cursor += rule.move_offset;
        self.state = rule.next_state;
    }

    fn count_ones(&self) -> usize {
        self.tape.values().filter(|&&v| v == true).count()
    }
}

fn get_states_and_starting_state() -> (Vec<State>, usize) {
    let states = vec![
        State {
            if_false: Rule {
                write_value: true,
                move_offset: 1,
                next_state: 1,
            },
            if_true: Rule {
                write_value: false,
                move_offset: -1,
                next_state: 2,
            }
        },
        State {
            if_false: Rule {
                write_value: true,
                move_offset: -1,
                next_state: 0,
            },
            if_true: Rule {
                write_value: true,
                move_offset: 1,
                next_state: 2,
            }
        },
        State {
            if_false: Rule {
                write_value: true,
                move_offset: 1,
                next_state: 0,
            },
            if_true: Rule {
                write_value: false,
                move_offset: -1,
                next_state: 3,
            }
        },
        State {
            if_false: Rule {
                write_value: true,
                move_offset: -1,
                next_state: 4,
            },
            if_true: Rule {
                write_value: true,
                move_offset: -1,
                next_state: 2,
            }
        },
        State {
            if_false: Rule {
                write_value: true,
                move_offset: 1,
                next_state: 5,
            },
            if_true: Rule {
                write_value: true,
                move_offset: 1,
                next_state: 0,
            }
        },
        State {
            if_false: Rule {
                write_value: true,
                move_offset: 1,
                next_state: 0,
            },
            if_true: Rule {
                write_value: true,
                move_offset: 1,
                next_state: 4,
            }
        },
    ];

    (states, 0)
}

fn main() {
    let (states, starting_state) = get_states_and_starting_state();
    let mut machine = TuringMachine::new(states, starting_state);
    for _ in 0..12134527 {
        machine.run_step();
    }

    println!("The answer to Part 1 is {}", machine.count_ones());
}
