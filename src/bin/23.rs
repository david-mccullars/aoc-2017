extern crate advent_of_code;

use advent_of_code::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

lazy_static::lazy_static! {
    static ref primes_txt: &'static str = include_str!("primes.txt");
    static ref PRIMES: HashSet<isize> = HashSet::from_iter(
        primes_txt.lines().map(|n| n.parse::<isize>().unwrap())
    );
}

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<usize> {
    let mut tablet = Tablet::new(parse(input));
    Some(tablet.run())
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut tablet = Tablet::new(parse(input));
    tablet.reg_set('a', 1);
    // Reverse engineer that lines 10-23 check if register b is prime
    // If it is not prime, it sets register f to 0
    tablet.instructions[10] = Inst::NotPrime('f', Arg::Register('b'));
    tablet.instructions[11] = Inst::Set('g', Arg::Value(0));
    tablet.instructions[12] = Inst::Jnz(Arg::Value(1), Arg::Value(23 - 12));
    tablet.run();
    Some(tablet.reg(&Arg::Register('h')))
}

fn parse(input: &str) -> Vec<Inst> {
    let arg = parser!({
        c:lower => Arg::Register(c),
        v:isize => Arg::Value(v),
    });
    parser!(lines({
        "set " x:lower " " y:arg  => Inst::Set(x, y),
        "sub " x:lower " " y:arg  => Inst::Sub(x, y),
        "mul " x:lower " " y:arg  => Inst::Mul(x, y),
        "jnz " x:arg " " y:arg  => Inst::Jnz(x, y),
    }))
    .parse(input)
    .expect("Failed to parse")
}

enum Arg {
    Register(char),
    Value(isize),
}

enum Inst {
    Set(char, Arg),
    Sub(char, Arg),
    Mul(char, Arg),
    Jnz(Arg, Arg),
    NotPrime(char, Arg),
}

struct Tablet {
    instructions: Vec<Inst>,
    registers: HashMap<char, isize>,
    pos: isize,
}

impl Tablet {
    fn new(instructions: Vec<Inst>) -> Self {
        Self {
            instructions,
            registers: HashMap::new(),
            pos: 0,
        }
    }

    fn reg(&self, arg: &Arg) -> isize {
        match arg {
            Arg::Register(c) => *self.registers.get(&c).unwrap_or(&0),
            Arg::Value(v) => *v,
        }
    }

    fn reg_set(&mut self, c: char, v: isize) {
        *self.registers.entry(c).or_default() = v;
    }

    fn run(&mut self) -> usize {
        let mut multiplies = 0;
        while (0..(self.instructions.len() as isize)).contains(&self.pos) {
            match &self.instructions[self.pos as usize] {
                Inst::Set(x, y) => {
                    self.reg_set(*x, self.reg(y));
                }
                Inst::Sub(x, y) => {
                    self.reg_set(*x, self.reg(&Arg::Register(*x)) - self.reg(y));
                }
                Inst::Mul(x, y) => {
                    multiplies += 1;
                    self.reg_set(*x, self.reg(&Arg::Register(*x)) * self.reg(y));
                }
                Inst::Jnz(x, y) => {
                    if self.reg(x) != 0 {
                        self.pos += self.reg(y) - 1;
                    }
                }
                // Set register x to 0 unless the value y is prime
                Inst::NotPrime(x, y) => {
                    if !PRIMES.contains(&self.reg(y)) {
                        self.reg_set(*x, 0);
                    }
                }
            }
            self.pos += 1;
        }
        multiplies
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3969));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(917));
    }
}
