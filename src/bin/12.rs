extern crate advent_of_code;

use advent_of_code::*;
use pathfinding::prelude::bfs_reach;
use std::collections::{BTreeSet, HashMap};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let programs = parse(input);
    Some(bfs_reach(&0, |id| programs.get(&id).unwrap()).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let programs = parse(input);
    let mut groups = 0;
    let mut ids: BTreeSet<usize> = BTreeSet::from_iter(programs.keys().copied());
    while let Some(next_id) = ids.pop_first() {
        for id in bfs_reach(&next_id, |id| programs.get(&id).unwrap()) {
            ids.remove(&id);
        }
        groups += 1;
    }
    Some(groups)
}

fn parse(input: &str) -> HashMap<usize, Vec<usize>> {
    parser!(hash_map(lines(usize " <-> " repeat_sep(usize, ", "))))
        .parse(input)
        .expect("Failed to parse")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
