extern crate advent_of_code;

use advent_of_code::*;
use itertools::Itertools;
use pathfinding::prelude::bfs_reach;

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<usize> {
    bridges(input).map(|b| b.strength).max()
}

pub fn part_two(input: &str) -> Option<usize> {
    bridges(input)
        .sorted_by_key(|b| (b.length, b.strength))
        .rev()
        .next()
        .map(|b| b.strength)
}

struct Bridge {
    strength: usize,
    length: usize,
}

impl Bridge {
    fn from(bridge: &[usize], pairs: &[(usize, usize)]) -> Self {
        let strength: usize = bridge
            .iter()
            .map(|i| {
                let (a, b) = pairs[*i];
                a + b
            })
            .sum();
        let length = bridge.len();
        Bridge { strength, length }
    }
}

fn bridges(input: &str) -> impl Iterator<Item = Bridge> {
    let pairs = parser!(lines(usize "/" usize))
        .parse(input)
        .expect("Failed to parse");
    let p2 = pairs.clone();

    let empty: (Vec<usize>, usize) = (vec![], 0);
    let all = bfs_reach(empty, move |node| {
        let mut bridges: Vec<(Vec<usize>, usize)> = vec![];
        for (i, pair) in pairs.iter().enumerate() {
            if !node.0.contains(&i) && (node.1 == pair.0 || node.1 == pair.1) {
                let mut bridge = node.0.clone();
                bridge.push(i);
                bridges.push((bridge, if node.1 == pair.0 { pair.1 } else { pair.0 }));
            }
        }
        bridges
    });

    all.map(move |(bridge, _)| Bridge::from(&bridge, &p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
    }
}
