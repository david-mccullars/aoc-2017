extern crate advent_of_code;

use advent_of_code::*;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<usize> {
    let mut turing = Turing::parse(input);
    turing.run_to_next_diagnostic();
    Some(turing.tape.len())
}

pub fn part_two(_input: &str) -> Option<&str> {
    Some("CLAIM THE FINAL GOLD STAR!!!")
}

#[derive(Debug)]
struct Op {
    input: bool,
    output: bool,
    cursor_delta: isize,
    next: char,
}

#[derive(Debug)]
struct Turing {
    tape: HashSet<isize>,
    cursor: isize,
    state: char,
    rules: HashMap<(char, bool), Op>,
    steps: usize,
}

impl Turing {
    fn parse(input: &str) -> Self {
        let b = parser!({ "1" => true, "0" => false });
        let lr = parser!({ "left" => -1, "right" => 1 });
        let rule = parser!(
            i:line("  If the current value is " b ":")
            o:line("    - Write the value " b ".")
            c:line("    - Move one slot to the " lr ".")
            n:line("    - Continue with state " upper ".")
            => Op { input: i, output: o, cursor_delta: c, next: n }
        );
        parser!(
            i:section(
                line("Begin in state " upper ".")
                line("Perform a diagnostic checksum after " usize " steps.")
            )
            t:sections(
                s:line("In state " upper ":")
                r0:rule
                r1:rule
                => {
                    assert_eq!(r0.input, false);
                    assert_eq!(r1.input, true);
                    [((s, false), r0), ((s, true), r1)]
                }
            )
            => Self {
                tape: HashSet::new(),
                cursor: 0,
                state: i.0,
                rules: HashMap::from_iter(t.into_iter().flatten()),
                steps: i.1,
            }
        )
        .parse(input)
        .expect("Failed to parse")
    }

    fn val(&self) -> bool {
        self.tape.contains(&self.cursor)
    }

    fn set(&mut self, v: bool) {
        if v {
            self.tape.insert(self.cursor);
        } else {
            self.tape.remove(&self.cursor);
        }
    }

    fn run_to_next_diagnostic(&mut self) {
        for _ in 0..self.steps {
            let op = self.rules.get(&(self.state, self.val())).unwrap();
            let cursor_delta = op.cursor_delta;
            let next = op.next;
            self.set(op.output);
            self.cursor += cursor_delta;
            self.state = next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
