extern crate advent_of_code;

use advent_of_code::*;
use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    Some(count_valid(input, uniq_len_one))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(count_valid(input, uniq_len_two))
}

fn count_valid(input: &str, uniq_len: impl Fn(&[String]) -> usize) -> usize {
    parser!(lines(repeat_sep(string(alpha+), " ")))
        .parse(input)
        .expect("Failed to parse")
        .into_iter()
        .filter(|passphrase| passphrase.len() == uniq_len(passphrase))
        .count()
}

fn uniq_len_one(v: &[String]) -> usize {
    v.iter().collect::<HashSet<_>>().len()
}

fn uniq_len_two(v: &[String]) -> usize {
    let charsort = |s: &String| s.chars().sorted().join("");
    v.iter().map(charsort).collect::<HashSet<_>>().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(3));
    }
}
