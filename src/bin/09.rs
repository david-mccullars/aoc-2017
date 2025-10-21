extern crate advent_of_code;

use advent_of_code::*;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    Some(parse::<GroupScorer>(input).score)
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(parse::<GarbageCounter>(input).chars)
}

fn parse<H: Handler + Default>(input: &str) -> H {
    let mut handler = H::default();
    let mut in_garbage = false;
    let mut escaping = false;
    for c in input.trim().chars() {
        match (c, in_garbage, escaping) {
            ('<', false, false) => {
                in_garbage = true;
            }
            ('>', true, false) => {
                in_garbage = false;
            }
            ('!', true, false) => {
                escaping = true;
            }
            (_, true, true) => {
                escaping = false;
            }
            (_, true, false) => {
                handler.after_garbage();
            }
            ('{', false, false) => {
                handler.group_start();
            }
            ('}', false, false) => {
                handler.group_end();
            }
            (_, false, false) => {}
            _ => {
                panic!("Invalid state: {} {} {}", c, in_garbage, escaping);
            }
        }
    }
    handler
}

trait Handler {
    fn group_start(&mut self) {}
    fn group_end(&mut self) {}
    fn after_garbage(&mut self) {}
}

#[derive(Default)]
struct GroupScorer {
    level: usize,
    score: usize,
}

impl Handler for GroupScorer {
    fn group_start(&mut self) {
        self.level += 1;
    }

    fn group_end(&mut self) {
        self.score += self.level;
        self.level -= 1;
    }
}

#[derive(Default)]
struct GarbageCounter {
    chars: usize,
}

impl Handler for GarbageCounter {
    fn after_garbage(&mut self) {
        self.chars += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("{}"), Some(1));
        assert_eq!(part_one("{{{}}}"), Some(6));
        assert_eq!(part_one("{{},{}}"), Some(5));
        assert_eq!(part_one("{{{},{},{{}}}}"), Some(16));
        assert_eq!(part_one("{<a>,<a>,<a>,<a>}"), Some(1));
        assert_eq!(part_one("{{<ab>},{<ab>},{<ab>},{<ab>}}"), Some(9));
        assert_eq!(part_one("{{<!!>},{<!!>},{<!!>},{<!!>}}"), Some(9));
        assert_eq!(part_one("{{<a!>},{<a!>},{<a!>},{<ab>}}"), Some(3));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("<>"), Some(0));
        assert_eq!(part_two("<random characters>"), Some(17));
        assert_eq!(part_two("<<<<>"), Some(3));
        assert_eq!(part_two("<{!>}>"), Some(2));
        assert_eq!(part_two("<!!>"), Some(0));
        assert_eq!(part_two("<!!!>>"), Some(0));
        assert_eq!(part_two("<{o\"i!a,<{i<a>"), Some(10));
    }
}
