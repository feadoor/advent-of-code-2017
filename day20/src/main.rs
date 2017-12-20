extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

const INPUT_PATH: &'static str = "inputs/input.txt";

type ThreeVec = (i64, i64, i64);

fn add(vec1: ThreeVec, vec2: ThreeVec) -> ThreeVec {
    (vec1.0 + vec2.0, vec1.1 + vec2.1, vec1.2 + vec2.2)
}

fn distance(vec1: ThreeVec, vec2: ThreeVec) -> f64 {
    magnitude((vec1.0 - vec2.0, vec1.1 - vec2.1, vec1.2 - vec2.2))
}

fn magnitude(vec: ThreeVec) -> f64 {
    ((vec.0 * vec.0 + vec.1 * vec.1 + vec.2 * vec.2) as f64).sqrt()
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Particle {
    position: ThreeVec,
    velocity: ThreeVec,
    acceleration: ThreeVec,
}

impl Particle {
    fn step(&self) -> Particle {
        let new_velocity = add(self.velocity, self.acceleration);
        let new_position = add(self.position, new_velocity);

        Particle {
            position: new_position,
            velocity: new_velocity,
            acceleration: self.acceleration,
        }
    }
}

fn parse_vector(vector_str: &str) -> ThreeVec {
    lazy_static! {
        static ref VECTOR_REGEX: Regex = Regex::new("<(-?\\d+),(-?\\d+),(-?\\d+)").unwrap();
    }

    let captures = VECTOR_REGEX.captures(vector_str).unwrap();
    (
        captures[1].parse().unwrap(),
        captures[2].parse().unwrap(),
        captures[3].parse().unwrap()
    )
}

fn parse_particle(particle_str: &str) -> Particle {
    lazy_static! {
        static ref PARTICLE_REGEX: Regex = Regex::new("p=(.*), v=(.*), a=(.*)").unwrap();
    }

    let captures = PARTICLE_REGEX.captures(particle_str).unwrap();
    Particle {
        position: parse_vector(&captures[1]),
        velocity: parse_vector(&captures[2]),
        acceleration: parse_vector(&captures[3]),
    }
}

fn get_particles_from_file(path: &str) -> Vec<Particle> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    reader.lines()
        .map(|line| line.unwrap())
        .map(|line| parse_particle(&line))
        .collect()
}

fn particle_with_smallest_acceleration(particles: &[Particle]) -> usize {
    particles.iter()
        .map(|p| magnitude(p.acceleration))
        .enumerate()
        .min_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
        .unwrap().0
}

fn particles_left_after_collisions(particles: &[Particle]) -> usize {
    let mut moving_particles: Vec<_> = particles.iter().map(|&p| Some(p)).collect();
    let mut distances = vec![Some(1.0 / 0.0); particles.len() * (particles.len() - 1)];

    loop {
        let mut next_distances: Vec<Option<f64>> = Vec::new();
        let mut destructions = Vec::new();

        for (idx, p1) in moving_particles.iter().enumerate() {
            for (jdx, p2) in moving_particles.iter().enumerate() {
                if idx != jdx {
                    if p1.is_some() && p2.is_some() {
                        if p1.unwrap().position == p2.unwrap().position {
                            destructions.push(idx);
                            destructions.push(jdx);
                        }
                        let particle_distance = distance(
                            p1.unwrap().position,
                            p2.unwrap().position
                        );
                        next_distances.push(Some(particle_distance));
                    } else {
                        next_distances.push(None);
                    }
                }
            }
        }

        for idx in destructions {
            moving_particles[idx] = None;
        }

        if next_distances.iter().zip(distances.iter())
            .all(|(&d1, d2)| d1 == None || d1.unwrap() >= d2.unwrap()) {
            return moving_particles.iter().filter(|x| x.is_some()).count();
        }

        distances = next_distances;
        for particle in moving_particles.iter_mut() {
            if particle.is_some() {
                *particle = Some(particle.unwrap().step());
            }
        }
    }
}

fn part1() {
    let particles = get_particles_from_file(INPUT_PATH);
    let closest_particle_idx = particle_with_smallest_acceleration(&particles);
    println!("The answer to Part 1 is {}", closest_particle_idx);
}

fn part2() {
    let particles = get_particles_from_file(INPUT_PATH);
    let left_after_collisions = particles_left_after_collisions(&particles);
    println!("The answer to Part 2 is {}", left_after_collisions);
}

fn main() {
    part1();
    part2();
}
