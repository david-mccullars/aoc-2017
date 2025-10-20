extern crate advent_of_code;

use advent_of_code::*;
use itertools::Itertools;
use std::collections::HashMap;
use topological_sort::TopologicalSort;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<String> {
    Some(Program::parse(input).name)
}

pub fn part_two(input: &str) -> Option<usize> {
    Program::parse(input).full_weight().err()
}

struct Program {
    name: String,
    weight: usize,
    towers: Vec<Program>,
}

impl Program {
    fn parse(input: &str) -> Self {
        let mut programs: HashMap<String, Program> = HashMap::new();

        let all = parser!(hash_map(lines(
            n:string(alpha+) " (" w:usize ")" " -> "? c:repeat_sep(string(alpha+), ", ") => (n, (w, c))
        ))).parse(input).expect("Failed to parse");

        let mut ts = TopologicalSort::<String>::new();
        for (name, (_, children)) in &all {
            for c in children {
                ts.add_dependency(c.clone(), name.clone());
            }
        }

        loop {
            let name = ts.pop().unwrap();
            let (weight, children) = all.get(&name).unwrap();
            let program = Self {
                name: name.clone(),
                weight: *weight,
                towers: children
                    .iter()
                    .map(|t| programs.remove(t).unwrap())
                    .collect(),
            };
            if ts.is_empty() {
                return program;
            } else {
                programs.insert(name, program);
            }
        }
    }

    fn full_weight(&self) -> Result<usize, usize> {
        if self.towers.is_empty() {
            return Ok(self.weight);
        }

        let mut tw = vec![];
        for t in &self.towers {
            tw.push(t.full_weight()?);
        }
        if let Some(i) = find_diff(&tw) {
            Err(self.towers[i].weight + tw[(i + 1) % tw.len()] - tw[i])
        } else {
            Ok(self.weight + tw[0] * self.towers.len())
        }
    }
}

fn find_diff(v: &Vec<usize>) -> Option<usize> {
    if v.len() < 3 {
        None
    } else if v[0] != v[1] && v[1] == v[2] {
        Some(0)
    } else {
        v.iter().position(|i| *i != v[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("tknk")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(60));
    }
}
