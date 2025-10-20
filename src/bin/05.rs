extern crate advent_of_code;

use advent_of_code::*;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    Some(process(input, |_| 1))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(process(input, |jmp| if jmp >= 3 { -1 } else { 1 }))
}

fn process(input: &str, adjust_jmp: impl Fn(isize) -> isize) -> usize {
    let mut instructions = parse(input);
    let mut i: isize = 0;
    let mut steps = 0;
    while i >= 0 && i < (instructions.len() as isize) {
        let jmp = instructions[i as usize];
        instructions[i as usize] += adjust_jmp(jmp);
        i += jmp;
        steps += 1;
    }
    steps
}

fn parse(input: &str) -> Vec<isize> {
    parser!(lines(isize)).parse(input).expect("Failed to parse")
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
        assert_eq!(result, Some(10));
    }
}
