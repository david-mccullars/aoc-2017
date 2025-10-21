use itertools::Itertools;

pub struct KnotHash {
    list: Vec<usize>,
    pos: usize,
    skip: usize,
}

impl KnotHash {
    pub fn new(size: usize) -> Self {
        Self {
            list: (0..size).collect(),
            pos: 0,
            skip: 0,
        }
    }

    pub fn run_round(&mut self, lengths: &[usize]) {
        for len in lengths {
            for i in 0..(len / 2) {
                let i1 = (self.pos + i) % self.list.len();
                let i2 = (self.pos + len - i - 1) % self.list.len();
                (self.list[i1], self.list[i2]) = (self.list[i2], self.list[i1]);
            }
            self.pos += len + self.skip;
            self.skip += 1;
        }
    }

    pub fn run_rounds(&mut self, lengths: &[usize], num: usize) {
        for _ in 0..num {
            self.run_round(lengths);
        }
    }

    pub fn get(&self, index: usize) -> usize {
        self.list[index]
    }

    /// Hash a string and return the dense hash as a vector of usize values
    pub fn hash(input: &str) -> impl Iterator<Item = usize> {
        let lengths: Vec<_> = input
            .trim()
            .chars()
            .map(|c| c as usize)
            .chain([17, 31, 73, 47, 23].into_iter())
            .collect();
        let mut hash = KnotHash::new(256);
        hash.run_rounds(&lengths, 64);
        (0..16).flat_map(move |i| {
            hash.list[(16 * i)..(16 * (i + 1))]
                .iter()
                .copied()
                .reduce(|i, j| i ^ j)
        })
    }

    /// Hash a string and return the hex-formatted hash string
    pub fn hashhex(input: &str) -> String {
        Self::hash(input).map(|n| format!("{:02x}", n)).join("")
    }
}
