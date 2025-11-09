extern crate advent_of_code;

use advent_of_code::*;
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap, HashSet};

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    let mut scanners = parse(input);
    let mut severity = 0;
    for ps in 0..=scanners.keys().copied().max().unwrap() {
        if let Some(s) = scanners.get(&ps) {
            if s.pos == 0 {
                severity += s.depth * s.range;
            }
        }
        for (_, s) in scanners.iter_mut() {
            s.advance();
        }
    }
    Some(severity)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut scanners = parse(input);
    let mut checks: BTreeMap<usize, HashSet<usize>> = BTreeMap::new();
    for s in scanners.values() {
        let m = s.range * 2 - 2;
        let d = ((m << 4) - s.depth) % m;
        checks.entry(m).or_default().insert(d);
    }
    for delay in 1.. {
        if !checks
            .iter()
            .any(|(m, checks)| checks.contains(&(delay % m)))
        {
            return Some(delay);
        }
    }
    None
}

fn parse(input: &str) -> HashMap<usize, Scanner> {
    parser!(hash_map(lines(d:usize ": " r:usize => (d, Scanner::new(d, r)))))
        .parse(input)
        .expect("Failed to parse")
}

#[derive(Clone)]
struct Scanner {
    depth: usize,
    range: usize,
    pos: usize,
}

impl Scanner {
    fn new(depth: usize, range: usize) -> Self {
        Self {
            depth,
            range,
            pos: 0,
        }
    }

    fn advance(&mut self) {
        self.pos = (self.pos + 1) % (self.range * 2 - 2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
