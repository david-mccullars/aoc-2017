extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::thread::spawn;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<isize> {
    let mut tablet = Tablet::new(0, parse(input));
    let mut last_sound = None;
    tablet.run(
        || None,
        |i| {
            last_sound = Some(i);
        },
        true,
    );
    last_sound
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut tablet0 = Tablet::new(0, parse(input));
    let mut tablet1 = Tablet::new(1, parse(input));

    let (output0, input0): (SyncSender<isize>, Receiver<isize>) = sync_channel(128);
    let (output1, input1): (SyncSender<isize>, Receiver<isize>) = sync_channel(128);

    let t0 = spawn(move || tablet0.run_sync(&input0, &output1));
    let t1 = spawn(move || tablet1.run_sync(&input1, &output0));

    t0.join();
    t1.join().ok()
}

fn parse(input: &str) -> Vec<Inst> {
    let arg = parser!({
        c:lower => Arg::Register(c),
        v:isize => Arg::Value(v),
    });
    parser!(lines({
        "snd " x:arg  => Inst::Snd(x),
        "set " x:lower " " y:arg  => Inst::Set(x, y),
        "add " x:lower " " y:arg  => Inst::Add(x, y),
        "mul " x:lower " " y:arg  => Inst::Mul(x, y),
        "mod " x:lower " " y:arg  => Inst::Mod(x, y),
        "rcv " x:lower  => Inst::Rcv(x),
        "jgz " x:arg " " y:arg  => Inst::Jgz(x, y),
    }))
    .parse(input)
    .expect("Failed to parse")
}

enum Arg {
    Register(char),
    Value(isize),
}

enum Inst {
    Snd(Arg),
    Set(char, Arg),
    Add(char, Arg),
    Mul(char, Arg),
    Mod(char, Arg),
    Rcv(char),
    Jgz(Arg, Arg),
}

struct Tablet {
    id: isize,
    instructions: Vec<Inst>,
    registers: HashMap<char, isize>,
    pos: isize,
}

impl Tablet {
    fn new(id: isize, instructions: Vec<Inst>) -> Self {
        Self {
            id,
            instructions,
            registers: HashMap::from([('p', id)]),
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

    fn run<I, O>(&mut self, mut input: I, mut output: O, rcv_check: bool)
    where
        I: FnMut() -> Option<isize>,
        O: FnMut(isize),
    {
        while (0..(self.instructions.len() as isize)).contains(&self.pos) {
            match &self.instructions[self.pos as usize] {
                Inst::Snd(x) => {
                    output(self.reg(x));
                }
                Inst::Set(x, y) => {
                    self.reg_set(*x, self.reg(y));
                }
                Inst::Add(x, y) => {
                    self.reg_set(*x, self.reg(&Arg::Register(*x)) + self.reg(y));
                }
                Inst::Mul(x, y) => {
                    self.reg_set(*x, self.reg(&Arg::Register(*x)) * self.reg(y));
                }
                Inst::Mod(x, y) => {
                    self.reg_set(*x, self.reg(&Arg::Register(*x)) % self.reg(y));
                }
                Inst::Rcv(x) => {
                    if !rcv_check || self.reg(&Arg::Register(*x)) != 0 {
                        if let Some(y) = input() {
                            self.reg_set(*x, y);
                        } else {
                            return;
                        }
                    }
                }
                Inst::Jgz(x, y) => {
                    if self.reg(x) > 0 {
                        self.pos += self.reg(y) - 1;
                    }
                }
            }
            self.pos += 1;
        }
    }

    fn run_sync(&mut self, input: &Receiver<isize>, output: &SyncSender<isize>) -> usize {
        let mut sends = 0;
        let input = || {
            input
                .recv_timeout(std::time::Duration::from_millis(10))
                .ok()
        };
        let output = |v| {
            output.try_send(v).expect("Failure sending output");
            sends += 1;
        };
        self.run(input, output, false);
        sends
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(3));
    }
}
