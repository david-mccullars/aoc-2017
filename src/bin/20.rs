extern crate advent_of_code;

use advent_of_code::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<usize> {
    let mut particles = Particles::parse(input);
    for _ in 0..1000 {
        particles.tick(|_, _| {});
    }
    particles
        .particles
        .iter()
        .enumerate()
        .map(|(id, p)| (md3(&p.pos, &(0, 0, 0)), id))
        .min()
        .map(|(_, id)| id)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut particles = Particles::parse(input);
    for _ in 0..100 {
        let mut by_pos: HashMap<Pos3, HashSet<usize>> = HashMap::new();
        particles.tick(|id, pos| {
            by_pos.entry(pos).or_default().insert(id);
        });
        for id in by_pos
            .values()
            .filter(|h| h.len() > 1)
            .flatten()
            .sorted()
            .rev()
        {
            particles.particles.remove(*id);
        }
    }
    Some(particles.particles.len())
}

type Pos3 = (isize, isize, isize);

fn add3(a: &Pos3, b: &Pos3) -> Pos3 {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn md3(a: &Pos3, b: &Pos3) -> isize {
    (b.0 - a.0).abs() + (b.1 - a.1).abs() + (b.2 - a.2).abs()
}

#[derive(Debug)]
struct Particle {
    pos: Pos3,
    vel: Pos3,
    acc: Pos3,
}

struct Particles {
    particles: Vec<Particle>,
}

impl Particles {
    fn parse(input: &str) -> Self {
        let trip = parser!("<" isize "," isize "," isize ">");
        let particles = parser!(lines(
            "p=" pos:trip ", v=" vel:trip ", a=" acc:trip
            => Particle { pos, vel, acc }
        ))
        .parse(input)
        .expect("Failed to parse");
        Self { particles }
    }

    fn tick(&mut self, mut track: impl FnMut(usize, Pos3)) {
        for (id, p) in self.particles.iter_mut().enumerate() {
            p.vel = add3(&p.vel, &p.acc);
            p.pos = add3(&p.pos, &p.vel);
            track(id, p.pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1));
    }
}
