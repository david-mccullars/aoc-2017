extern crate advent_of_code;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<usize> {
    Some(captcha(input, 1))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(captcha(input, input.trim().len() / 2))
}

pub fn captcha(input: &str, offset: usize) -> usize {
    let chars: Vec<char> = input.trim().chars().collect();
    chars
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(i, c)| (c == chars[(i + offset) % chars.len()]).then_some((c as usize) - 48))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("1122"), Some(3));
        assert_eq!(part_one("1111"), Some(4));
        assert_eq!(part_one("1234"), Some(0));
        assert_eq!(part_one("91212129\n"), Some(9));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("1212"), Some(6));
        assert_eq!(part_two("1221"), Some(0));
        assert_eq!(part_two("123425"), Some(4));
        assert_eq!(part_two("123123"), Some(12));
        assert_eq!(part_two("12131415\n"), Some(4));
    }
}
