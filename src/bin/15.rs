extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<usize> {
    let (a, b) = parse(input);
    let mut pairs = a.zip(b);
    Some(pairs.take(40_000_000).filter(|(a, b)| a == b).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (a, b) = parse(input);
    let mut pairs = a.filter(|a| a % 4 == 0).zip(b.filter(|b| b % 8 == 0));
    Some(pairs.take(5_000_000).filter(|(a, b)| a == b).count())
}

fn parse(input: &str) -> (Generator, Generator) {
    parser!(
        line("Generator A starts with " a:usize => generator(a, 16807))
        line("Generator B starts with " b:usize => generator(b, 48271))
    )
    .parse(input)
    .expect("Failed to parse")
}

fn generator(start: usize, factor: usize) -> Generator {
    Generator {
        prev_value: start,
        factor,
    }
}

struct Generator {
    prev_value: usize,
    factor: usize,
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.prev_value = (self.prev_value * self.factor) % 2147483647;
        Some(self.prev_value % 65536)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(588));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(309));
    }
}
