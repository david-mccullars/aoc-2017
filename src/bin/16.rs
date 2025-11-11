extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(16);

#[cfg(test)]
const PROGRAMS: usize = 5;
#[cfg(not(test))]
const PROGRAMS: usize = 16;

pub fn part_one(input: &str) -> Option<String> {
    let moves = parse(input);
    let mut dp = DancingPrograms::new();
    dp.dance(&moves);
    Some(dp.string())
}

pub fn part_two(input: &str) -> Option<String> {
    let moves = parse(input);
    let mut dp = DancingPrograms::new();

    let mut seen = HashMap::new();
    let mut cache = true;
    let mut i = 0;
    while i < 1000000000 {
        if cache {
            if let Some(prev_i) = seen.insert(dp.id(), i) {
                let delta = i - prev_i;
                while i + delta < 1000000000 {
                    i += delta;
                }
                cache = false;
            }
        }
        dp.dance(&moves);
        i += 1;
    }
    Some(dp.string())
}

fn parse(input: &str) -> Vec<DanceMove> {
    parser!(line(repeat_sep({
        "s" x:usize => DanceMove::Spin(x),
        "x" a:usize "/" b:usize => DanceMove::Exchange(a, b),
        "p" a:lower "/" b:lower => DanceMove::Partner(a, b),
    }, ",")))
    .parse(input)
    .expect("Failed to parse")
}

enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

struct DancingPrograms {
    programs: Vec<char>,
    head: usize,
}

impl DancingPrograms {
    fn new() -> Self {
        Self {
            programs: ('a'..).take(PROGRAMS).collect(),
            head: 0,
        }
    }

    fn id(&self) -> (usize, Vec<char>) {
        (self.head, self.programs.clone())
    }

    fn dance(&mut self, moves: &[DanceMove]) {
        for dm in moves {
            match dm {
                DanceMove::Spin(x) => {
                    self.head = (self.head + self.programs.len() - x) % self.programs.len();
                }
                DanceMove::Exchange(a, b) => {
                    self.swap(self.index(a), self.index(b));
                }
                DanceMove::Partner(a, b) => {
                    self.swap(self.pos(a), self.pos(b));
                }
            }
        }
    }

    fn index(&self, i: &usize) -> usize {
        (self.head + i) % self.programs.len()
    }

    fn pos(&self, c: &char) -> usize {
        self.programs.iter().position(|c2| c == c2).unwrap()
    }

    fn swap(&mut self, ai: usize, bi: usize) {
        (self.programs[ai], self.programs[bi]) = (self.programs[bi], self.programs[ai]);
    }

    fn string(&self) -> String {
        self.programs[self.head..]
            .iter()
            .chain(self.programs[0..self.head].iter())
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("baedc")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("abcde")));
    }
}
