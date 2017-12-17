fn spinlock(steps: usize, step_size: usize) -> (Vec<usize>, usize) {
    let mut lock = vec![0];
    let mut current_position = 0;

    for step in 0..steps {
        current_position = (current_position + step_size) % lock.len();
        lock.insert(current_position + 1, step + 1);
        current_position += 1;
    }

    (lock, current_position)
}

fn spinlock_value_after_zero(steps: usize, step_size: usize) -> usize {
    let mut current_position = 0;
    let mut answer = 0;

    for step in 0..steps {
        current_position = (current_position + step_size) % (step + 1);
        current_position += 1;

        if current_position == 1 {
            answer = step + 1;
        }
    }

    answer
}

fn part1() {
    let (spinlock, position) = spinlock(2017, 316);
    let answer = spinlock[(position + 1) % spinlock.len()];
    println!("The answer to Part 1 is {}", answer);
}

fn part2() {
    let answer = spinlock_value_after_zero(50_000_000, 316);
    println!("The answer to Part 2 is {}", answer);
}

fn main() {
    part1();
    part2();
}
