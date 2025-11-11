extern crate advent_of_code;

use advent_of_code::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<String> {
    let g = Diagram::parse(input);
    let mut path = String::from("");
    g.follow(|c| path.push(c));
    Some(path)
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Diagram::parse(input).follow(|_| {}))
}

#[derive(Default)]
struct Diagram {
    paths: HashSet<Pos>,
    markers: HashMap<Pos, char>,
}

impl Diagram {
    fn parse(input: &str) -> Self {
        let mut diagram = Self::default();
        let grid = parser!(grid_of(" |+-ABCDEFGHIJKLMNOPQRSTUVWXYZ"))
            .parse(input)
            .expect("Failed to parse");
        for (c, s) in grid.map.into_iter() {
            match c {
                'A'..'Z' => {
                    let pos = s.into_iter().next().unwrap();
                    diagram.markers.insert(pos.clone(), c);
                    diagram.paths.insert(pos);
                }
                _ => {
                    diagram.paths = diagram.paths.union(&s).copied().collect();
                }
            }
        }
        diagram
    }

    fn start(&self) -> Pos {
        *self.paths.iter().sorted_by_key(|pos| pos.1).next().unwrap()
    }

    fn follow(&self, mut on_marker: impl FnMut(char)) -> usize {
        let mut pos = self.start();
        let mut dir = Direction::South;
        for step in 1.. {
            if let Some(c) = self.markers.get(&pos) {
                on_marker(*c);
            }

            let pos2 = dir.forward_from(&pos);
            if self.paths.contains(&pos2) {
                pos = pos2;
            } else {
                if let Some((dir2, pos2)) = [dir.turn_left(), dir.turn_right()]
                    .into_iter()
                    .filter_map(|dir2| {
                        let pos2 = dir2.forward_from(&pos);
                        self.paths.contains(&pos2).then_some((dir2, pos2))
                    })
                    .next()
                {
                    pos = pos2;
                    dir = dir2;
                } else {
                    return step;
                }
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("ABCDEF")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(38));
    }
}
