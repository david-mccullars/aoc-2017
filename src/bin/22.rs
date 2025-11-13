extern crate advent_of_code;

use advent_of_code::*;
use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<usize> {
    let mut vc = VirusCarrier::parse(input);
    let mut infections = 0;
    for _ in 0..10000 {
        if vc.burst() {
            infections += 1;
        }
    }
    Some(infections)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut vc = VirusCarrier::parse(input);
    let mut infections = 0;
    for _ in 0..10000000 {
        if vc.evolved_burst() {
            infections += 1;
        }
    }
    Some(infections)
}

#[derive(Debug)]
enum State {
    Weakened,
    Infected,
    Flagged,
}

#[derive(Debug)]
struct VirusCarrier {
    pos: Pos,
    dir: Direction,
    nodes: HashMap<Pos, State>,
}

impl VirusCarrier {
    fn parse(input: &str) -> Self {
        let mut grid = parser!(grid_of(".#"))
            .parse(input)
            .expect("Failed to parse");
        let nodes = grid
            .take_all('#')
            .into_iter()
            .map(|pos| (pos, State::Infected))
            .collect();
        Self {
            pos: (grid.bounds.0 / 2, grid.bounds.1 / 2),
            dir: Direction::North,
            nodes,
        }
    }

    fn burst(&mut self) -> bool {
        let new_infection = match self.nodes.get(&self.pos) {
            Some(State::Infected) => {
                self.dir = self.dir.turn_right();
                self.nodes.remove(&self.pos);
                false
            }
            None => {
                self.dir = self.dir.turn_left();
                self.nodes.insert(self.pos.clone(), State::Infected);
                true
            }
            _ => panic!("Not supported"),
        };
        self.pos = self.dir.forward_from(&self.pos);
        new_infection
    }

    fn evolved_burst(&mut self) -> bool {
        let new_infection = match self.nodes.entry(self.pos.clone()) {
            Entry::Occupied(mut e) => match e.get() {
                State::Infected => {
                    self.dir = self.dir.turn_right();
                    *e.get_mut() = State::Flagged;
                    false
                }
                State::Weakened => {
                    *e.get_mut() = State::Infected;
                    true
                }
                State::Flagged => {
                    self.dir = self.dir.invert();
                    e.remove_entry();
                    false
                }
            },
            Entry::Vacant(mut e) => {
                self.dir = self.dir.turn_left();
                e.insert(State::Weakened);
                false
            }
        };
        self.pos = self.dir.forward_from(&self.pos);
        new_infection
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5587));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2511944));
    }
}
