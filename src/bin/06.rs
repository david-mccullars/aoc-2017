extern crate advent_of_code;

use advent_of_code::*;
use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    MemoryBanks::parse(input)
        .reallocate_until_repeat()
        .map(|cycle| *cycle.end())
}

pub fn part_two(input: &str) -> Option<usize> {
    MemoryBanks::parse(input)
        .reallocate_until_repeat()
        .map(|cycle| cycle.end() - cycle.start())
}

struct MemoryBanks {
    banks: Vec<usize>,
    len: usize,
}

impl MemoryBanks {
    fn parse(input: &str) -> Self {
        let banks = parser!(line(repeat_sep(usize, "\t")))
            .parse(input)
            .expect("Failed to parse");
        let len = banks.len();
        Self { banks, len }
    }

    fn reallocate_until_repeat(&mut self) -> Option<std::ops::RangeInclusive<usize>> {
        let mut seen = HashMap::new();
        for step in 0.. {
            if let Some(prev_step) = seen.insert(self.banks.clone(), step) {
                return Some(prev_step..=step);
            }
            self.reallocate()
        }
        None
    }

    fn reallocate(&mut self) {
        let i = self.index_for_max();
        let removed = self.banks[i];
        self.banks[i] = 0;
        for j in 0..removed {
            self.banks[(i + j + 1) % self.len] += 1;
        }
    }

    fn index_for_max(&self) -> usize {
        self.banks
            .iter()
            .enumerate()
            .map(|(a, b)| (-(*b as isize), a))
            .sorted()
            .next()
            .unwrap()
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
