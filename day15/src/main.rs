struct Generator {
    multiplier: u64,
    current: u64,
    divisor_check: u64,
}

impl Generator {

    fn new(multiplier: u64, start: u64, divisor: u64) -> Generator {
        Generator {
            multiplier,
            current: start,
            divisor_check: divisor,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        loop {
            self.current = (self.current * self.multiplier) % 2147483647;
            if self.current % self.divisor_check == 0 {
                return Some(self.current);
            }
        }
    }
}

fn matching_pairs(gen_a: Generator, gen_b: Generator, iterations: usize) -> usize {
    gen_a.zip(gen_b)
        .take(iterations)
        .filter(|&(a, b)| a as u16 == b as u16)
        .count()
}

fn part1() {
    let generator_a = Generator::new(16807, 618, 1);
    let generator_b = Generator::new(48271, 814, 1);

    let answer = matching_pairs(generator_a, generator_b, 40_000_000);
    println!("The answer to Part 1 is {}", answer);
}

fn part2() {
    let generator_a = Generator::new(16807, 618, 4);
    let generator_b = Generator::new(48271, 814, 8);

    let answer = matching_pairs(generator_a, generator_b, 5_000_000);
    println!("The answer to Part 1 is {}", answer);
}

fn main() {
    part1();
    part2();
}
