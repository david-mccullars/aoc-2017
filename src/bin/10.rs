extern crate advent_of_code;

use advent_of_code::*;
use itertools::Itertools;

advent_of_code::solution!(10);

#[cfg(test)]
const SIZE1: usize = 5;
#[cfg(not(test))]
const SIZE1: usize = 256;

pub fn part_one(input: &str) -> Option<usize> {
    let lengths = parser!(line(repeat_sep(usize, ",")))
        .parse(input)
        .expect("Failed to parse");
    let mut hash = KnotHash::new(SIZE1);
    hash.run_round(&lengths);
    Some(hash.list[0] * hash.list[1])
}

pub fn part_two(input: &str) -> Option<String> {
    let lengths: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c as usize)
        .chain([17, 31, 73, 47, 23].into_iter())
        .collect();
    let mut hash = KnotHash::new(256);
    hash.run_rounds(&lengths, 64);
    Some(
        hash.dense_hash()
            .into_iter()
            .map(|n| format!("{:02x}", n))
            .join(""),
    )
}

struct KnotHash {
    size: usize,
    list: Vec<usize>,
    pos: usize,
    skip: usize,
}

impl KnotHash {
    fn new(size: usize) -> Self {
        Self {
            size,
            list: (0..size).collect(),
            pos: 0,
            skip: 0,
        }
    }

    fn run_round(&mut self, lengths: &[usize]) {
        for len in lengths {
            for i in (0..(len / 2)) {
                let i1 = (self.pos + i) % self.size;
                let i2 = (self.pos + len - i - 1) % self.size;
                (self.list[i1], self.list[i2]) = (self.list[i2], self.list[i1]);
            }
            self.pos += len + self.skip;
            self.skip += 1;
        }
    }

    fn run_rounds(&mut self, lengths: &[usize], num: usize) {
        for _ in 0..num {
            self.run_round(lengths);
        }
    }

    fn dense_hash(&self) -> Vec<usize> {
        (0..16)
            .flat_map(|i| {
                self.list[(16 * i)..(16 * (i + 1))]
                    .iter()
                    .copied()
                    .reduce(|i, j| i ^ j)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("3,4,1,5"), Some(12));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(""),
            Some(String::from("a2582a3a0e66e6e86e3812dcb672a272"))
        );
        assert_eq!(
            part_two("AoC 2017"),
            Some(String::from("33efeb34ea91902bb2f59c9920caa6cd"))
        );
        assert_eq!(
            part_two("1,2,3"),
            Some(String::from("3efbe78a8d82f29979031a4aa0b16a9d"))
        );
        assert_eq!(
            part_two("1,2,4\n"),
            Some(String::from("63960835bcdc130f0b66d7ff4f6a5a8e"))
        );
    }
}
