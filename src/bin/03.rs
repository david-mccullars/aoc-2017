extern crate advent_of_code;

use advent_of_code::*;
use std::collections::HashMap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<isize> {
    let pos = spiral_pos(parse(input));
    Some(manhattan_distance(&pos, &(0, 0)))
}

pub fn part_two(input: &str) -> Option<isize> {
    let max = parse(input);
    let mut values = HashMap::new();
    values.insert((0, 0), 1);
    for n in 2.. {
        let pos = spiral_pos(n);
        let v = [-1, 0, 1]
            .into_iter()
            .flat_map(|dx| {
                [-1, 0, 1]
                    .into_iter()
                    .filter(move |dy| dx != 0 || *dy != 0)
                    .map(move |dy| (dx, dy))
            })
            .map(|(dx, dy)| values.get(&(pos.0 + dx, pos.1 + dy)).unwrap_or(&0))
            .sum();
        if v > max {
            return Some(v);
        }
        values.insert(pos, v);
    }
    None
}

fn spiral_pos(n: isize) -> Pos {
    let r = (((n as f32).sqrt() - 1.0) / 2.0).ceil() as isize;
    let bottom_right = (r * 2 + 1).pow(2);
    if n >= bottom_right - 2 * r {
        (n - bottom_right + r, -r)
    } else if n > bottom_right - 4 * r {
        (-r, (bottom_right - 3 * r) - n)
    } else if n > bottom_right - 6 * r {
        ((bottom_right - 5 * r) - n, r)
    } else if n > bottom_right - 8 * r {
        (r, n - (bottom_right - 7 * r))
    } else {
        panic!("NOPE")
    }
}

fn parse(input: &str) -> isize {
    input.trim().parse::<isize>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("1"), Some(0));
        assert_eq!(part_one("12"), Some(3));
        assert_eq!(part_one("23"), Some(2));
        assert_eq!(part_one("1024\n"), Some(31));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("1"), Some(2));
        assert_eq!(part_two("12"), Some(23));
        assert_eq!(part_two("23"), Some(25));
        assert_eq!(part_two("1024\n"), Some(1968));
    }
}
