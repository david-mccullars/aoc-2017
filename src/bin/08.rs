extern crate advent_of_code;

use advent_of_code::*;
use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<isize> {
    Some(find_maxes(input).0)
}

pub fn part_two(input: &str) -> Option<isize> {
    Some(find_maxes(input).1)
}

fn find_maxes(input: &str) -> (isize, isize) {
    let mut overall_max = 0;
    let mut registers: HashMap<String, isize> = HashMap::new();
    for (reg, inc, qty, cond_reg, cond_op, cond_qty) in parse(input) {
        if cond_op.eval(*registers.get(&cond_reg).unwrap_or(&0), cond_qty) {
            *registers.entry(reg).or_default() += if inc { qty } else { -qty };
            overall_max = std::cmp::max(overall_max, registers.values().copied().max().unwrap());
        }
    }
    (registers.values().copied().max().unwrap(), overall_max)
}

#[derive(Debug)]
enum Op {
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
    Neq,
}

impl Op {
    fn eval(&self, q1: isize, q2: isize) -> bool {
        match self {
            Op::Gt => q1 > q2,
            Op::Gte => q1 >= q2,
            Op::Lt => q1 < q2,
            Op::Lte => q1 <= q2,
            Op::Eq => q1 == q2,
            Op::Neq => q1 != q2,
        }
    }
}

fn parse(input: &str) -> Vec<(String, bool, isize, String, Op, isize)> {
    let op = parser!({
        " > " => Op::Gt,
        " >= " => Op::Gte,
        " < " => Op::Lt,
        " <= " => Op::Lte,
        " == " => Op::Eq,
        " != " => Op::Neq,
    });
    parser!(lines(
        string(alpha+) { " inc " => true, " dec " => false } isize " if " string(alpha+) op isize
    ))
    .parse(input)
    .expect("Failed to parse")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
