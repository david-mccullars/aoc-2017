extern crate advent_of_code;

use advent_of_code::*;
use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    checksums(input, |row| row[row.len() - 1] - row[0])
}

pub fn part_two(input: &str) -> Option<usize> {
    checksums(input, |row| {
        row.iter()
            .combinations(2)
            .find_map(|pair| (pair[1] % pair[0] == 0).then_some(pair[1] / pair[0]))
            .unwrap()
    })
}

fn checksums(input: &str, per_row: impl Fn(Vec<usize>) -> usize) -> Option<usize> {
    Some(parse(input).into_iter().map(|row| per_row(row)).sum())
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    parser!(lines(row:repeat_sep(usize, "\t") => row.into_iter().sorted().collect()))
        .parse(input)
        .expect("Failed to parse")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9));
    }
}
