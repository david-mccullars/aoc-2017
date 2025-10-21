extern crate advent_of_code;

use advent_of_code::*;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let pos = parse(input)
        .into_iter()
        .fold((0, 0), |pos, dir| dir.forward_from(&pos));
    Some(hexdist(&pos))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut maxdist = 0;
    let mut pos = (0, 0);
    for dir in parse(input) {
        pos = dir.forward_from(&pos);
        maxdist = std::cmp::max(maxdist, hexdist(&pos));
    }
    Some(maxdist)
}

fn parse(input: &str) -> Vec<HexDir> {
    parser!(line(repeat_sep({
        "sw" => HexDir::SouthWest,
        "se" => HexDir::SouthEast,
        "s" => HexDir::South,
        "nw" => HexDir::NorthWest,
        "ne" => HexDir::NorthEast,
        "n" => HexDir::North,
    }, ",")))
    .parse(input)
    .expect("Failed to parse")
}

#[derive(Debug)]
enum HexDir {
    SouthEast,
    SouthWest,
    South,
    NorthEast,
    NorthWest,
    North,
}

pub static DIRECTIONS: [HexDir; 6] = [
    HexDir::SouthEast,
    HexDir::SouthWest,
    HexDir::South,
    HexDir::NorthEast,
    HexDir::NorthWest,
    HexDir::North,
];

impl HexDir {
    fn forward_from(&self, pos: &Pos) -> Pos {
        match self {
            HexDir::SouthEast => (pos.0 + 1, pos.1 + 1),
            HexDir::SouthWest => (pos.0 - 1, pos.1 + 1),
            HexDir::South => (pos.0, pos.1 + 2),
            HexDir::NorthEast => (pos.0 + 1, pos.1 - 1),
            HexDir::NorthWest => (pos.0 - 1, pos.1 - 1),
            HexDir::North => (pos.0, pos.1 - 2),
        }
    }
}

fn hexdist(pos: &Pos) -> usize {
    ((pos.0.abs() + pos.1.abs()) / 2) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("ne,ne,ne"), Some(3));
        assert_eq!(part_one("ne,ne,sw,sw"), Some(0));
        assert_eq!(part_one("ne,ne,s,s"), Some(2));
        assert_eq!(part_one("se,sw,se,sw,sw"), Some(3));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("ne,ne,ne"), Some(3));
        assert_eq!(part_two("ne,ne,sw,sw"), Some(2));
        assert_eq!(part_two("ne,ne,s,s"), Some(2));
        assert_eq!(part_two("se,sw,s,s,s,n,n,n,se,sw,sw"), Some(4));
    }
}
