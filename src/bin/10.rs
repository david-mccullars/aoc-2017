extern crate advent_of_code;

use advent_of_code::*;

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
    Some(hash.get(0) * hash.get(1))
}

pub fn part_two(input: &str) -> Option<String> {
    Some(KnotHash::hashhex(input))
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
