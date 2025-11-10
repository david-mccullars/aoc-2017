extern crate advent_of_code;

use advent_of_code::*;
use pathfinding::prelude::bfs_reach;
use std::collections::HashSet;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    Some(Disk::parse(input).used.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut disk = Disk::parse(input);
    let mut regions = 0;
    while let Some(pos) = disk.used.iter().copied().next() {
        for pos2 in disk.region(&pos) {
            disk.used.remove(&pos2);
        }
        regions += 1;
    }
    Some(regions)
}

struct Disk {
    used: HashSet<Pos>,
}

impl Disk {
    fn parse(input: &str) -> Self {
        let mut used = HashSet::new();
        for y in 0..128 {
            for (x, b) in hash(&format!("{}-{}", input.trim(), y)).enumerate() {
                if b {
                    used.insert(pos_from(x, y));
                }
            }
        }
        Self { used }
    }

    fn adjacent_used(&self, pos: &Pos) -> Vec<Pos> {
        DIRECTIONS
            .into_iter()
            .map(|dir| dir.forward_from(pos))
            .filter(|pos2| self.used.contains(pos2))
            .collect()
    }

    fn region(&self, pos: &Pos) -> Vec<Pos> {
        bfs_reach(*pos, |pos2| self.adjacent_used(pos2)).collect()
    }
}

fn hash(s: &str) -> impl Iterator<Item = bool> {
    KnotHash::hash(&s).flat_map(|i| (0..8).rev().map(move |n| ((i >> n) & 1) == 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8108));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1242));
    }
}
