extern crate advent_of_code;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<usize> {
    let step = parse(input);
    let mut list = vec![0];
    let mut pos = 0;
    for i in 1..2018 {
        pos = (pos + step) % list.len() + 1;
        list.insert(pos, i);
    }
    Some(list[(pos + 1) % list.len()])
}

pub fn part_two(input: &str) -> Option<usize> {
    let step = parse(input);
    let mut after_zero = None;
    let mut pos = 0;
    let mut len = 1;
    for i in 1..50000000 {
        pos = (pos + step) % len + 1;
        if pos == 1 {
            after_zero = Some(i);
        }
        len += 1;
    }
    after_zero
}

fn parse(input: &str) -> usize {
    input.trim().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(638));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1222153));
    }
}
