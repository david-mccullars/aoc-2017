extern crate advent_of_code;

use advent_of_code::*;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

advent_of_code::solution!(21);

const START_IMAGE: [Pos; 5] = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];

pub fn part_one(input: &str) -> Option<usize> {
    enhance(input, 5)
}

pub fn part_two(input: &str) -> Option<usize> {
    enhance(input, 18)
}

fn enhance(input: &str, count: usize) -> Option<usize> {
    let rules = parse(input);
    let mut image = Pattern {
        size: 3,
        on: HashSet::from(START_IMAGE),
    };

    for _ in 0..count {
        image = image.enhance(&rules);
    }
    Some(image.on.len())
}

fn parse(input: &str) -> HashMap<(usize, u64), Pattern> {
    let pat = parser!(s:repeat_sep(char_of(".#")+, "/") => Pattern::new(s));
    let parsed = parser!(lines(a:pat " => " b:pat))
        .parse(input)
        .expect("Failed to parse");

    let mut rules = HashMap::new();
    for (a, b) in parsed {
        for rule in a.arrangements() {
            rules.insert(rule, b.clone());
        }
    }
    rules
}

#[derive(Debug, Clone)]
struct Pattern {
    size: usize,
    on: HashSet<Pos>,
}

impl Pattern {
    fn new(lines: Vec<Vec<usize>>) -> Self {
        let size = lines.len();
        let on = lines
            .into_iter()
            .enumerate()
            .flat_map(move |(y, line)| {
                assert_eq!(line.len(), size);
                line.into_iter()
                    .enumerate()
                    .filter_map(move |(x, c)| (c == 1).then_some(pos_from(x, y)))
            })
            .collect();
        Self { size, on }
    }

    fn rotate(&self) -> Self {
        let s = self.size as isize;
        let on = self.on.iter().map(|pos| (s - pos.1 - 1, pos.0)).collect();
        Self {
            size: self.size,
            on,
        }
    }

    fn flip(&self) -> Self {
        let s = self.size as isize;
        let on = self.on.iter().map(|pos| (s - pos.0 - 1, pos.1)).collect();
        Self {
            size: self.size,
            on,
        }
    }

    fn arrangements(&self) -> HashSet<(usize, u64)> {
        let mut a = HashSet::new();
        let mut p = self.rotate();
        for _ in 0..4 {
            a.insert((self.size, p.to_i()));
            p = p.rotate();
        }
        p = p.flip();
        for _ in 0..4 {
            a.insert((self.size, p.to_i()));
            p = p.rotate();
        }
        a
    }

    fn to_i(&self) -> u64 {
        assert!(self.size <= 8);
        to_i(&self.on, 0..self.size, 0..self.size)
    }

    fn enhance(&self, rules: &HashMap<(usize, u64), Pattern>) -> Self {
        let mut size = 0;
        let mut on = HashSet::new();

        let s = if self.size % 2 == 0 { 2 } else { 3 };
        for gy in 0..(self.size / s) {
            for gx in 0..(self.size / s) {
                let i = to_i(&self.on, (gx * s)..(gx * s + s), (gy * s)..(gy * s + s));
                let pos = pos_from(gx * (s + 1), gy * (s + 1));
                let rule = rules.get(&(s, i)).unwrap();
                for pos2 in &rule.on {
                    //eprintln!("==> {:?} + {:?}", &pos, &pos2);
                    on.insert((pos.0 + pos2.0, pos.1 + pos2.1));
                }
                if gy == 0 {
                    size += rule.size;
                }
            }
        }

        Self { size, on }
    }
}

fn to_i(on: &HashSet<Pos>, xs: Range<usize>, ys: Range<usize>) -> u64 {
    let mut b = 0;
    for y in ys {
        for x in xs.clone() {
            b = b << 1;
            if on.contains(&pos_from(x, y)) {
                b += 1;
            }
        }
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhance() {
        let result = enhance(&advent_of_code::template::read_file("examples", DAY), 2);
        assert_eq!(result, Some(12));
    }
}
