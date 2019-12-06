use crate::{get_input, AdventError, AdventResult};
use std::collections::HashSet;

use std::cmp::{Eq, PartialEq};
use std::convert::TryFrom;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn manhattan_distance(self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

pub enum Direction {
    L,
    R,
    U,
    D,
}

impl TryFrom<Option<char>> for Direction {
    type Error = AdventError;

    fn try_from(c: Option<char>) -> AdventResult<Direction> {
        let dir = match c {
            Some('L') => Direction::L,
            Some('R') => Direction::R,
            Some('U') => Direction::U,
            Some('D') => Direction::D,
            _ => return Err(AdventError::InvalidValue),
        };
        Ok(dir)
    }
}

pub struct Instruction {
    direction: Direction,
    length: u64,
}

pub fn parse_instruction<T: AsRef<str>>(direction: T) -> AdventResult<Instruction> {
    let (dir, len) = direction.as_ref().split_at(1);
    let dir = Direction::try_from(dir.chars().next())?;
    let len = len.parse::<u64>()?;
    Ok(Instruction {
        direction: dir,
        length: len,
    })
}

pub fn parse_instructions<T: AsRef<str>>(input: &[T]) -> AdventResult<Vec<Instruction>> {
    input.iter().map(parse_instruction).collect()
}

pub fn load_input() -> AdventResult<(Vec<Instruction>, Vec<Instruction>)> {
    let input = get_input::<String>(3)?.data;
    let wire1 = parse_instructions(&input[0])?;
    let wire2 = parse_instructions(&input[1])?;
    Ok((wire1, wire2))
}

pub fn solve_part1() -> AdventResult<i64> {
    let (wire1, wire2) = load_input()?;
    let res = closest_crossing(&wire1, &wire2)?;
    println!("Result: {}", res);
    Ok(res)
}

pub fn solve_part2() -> AdventResult<u64> {
    let (wire1, wire2) = load_input()?;
    let res = cheapest_crossing(&wire1, &wire2)?;
    println!("Result: {}", res);
    Ok(res)
}

pub fn get_crossings(wire1: &[Instruction], wire2: &[Instruction]) -> AdventResult<HashSet<Coord>> {
    let wire1_path = wire_path(wire1)?;
    let wire2_path = wire_path(wire2)?;
    let crossings = wire1_path.intersection(&wire2_path).copied().collect();
    Ok(crossings)
}

pub fn closest_crossing(wire1: &[Instruction], wire2: &[Instruction]) -> AdventResult<i64> {
    let crossings = get_crossings(wire1, wire2)?;

    crossings
        .iter()
        .map(|c| c.manhattan_distance())
        .min()
        .ok_or(AdventError::InvalidValue)
}

pub fn cheapest_crossing(wire1: &[Instruction], wire2: &[Instruction]) -> AdventResult<u64> {
    let crossings = get_crossings(wire1, wire2)?;

    crossings
        .iter()
        .map(|c| steps_to_crossing(c, wire1).unwrap() + steps_to_crossing(c, wire2).unwrap())
        .min()
        .ok_or(AdventError::InvalidValue)
}

pub fn steps_to_crossing(crossing: &Coord, wire_instructions: &[Instruction]) -> Option<u64> {
    let mut steps = 0;
    let mut coord = Coord { x: 0, y: 0 };

    for instruction in wire_instructions.iter() {
        let (x_shift, y_shift) = match instruction.direction {
            Direction::L => (-1, 0),
            Direction::R => (1, 0),
            Direction::U => (0, 1),
            Direction::D => (0, -1),
        };

        for _ in 0..instruction.length {
            steps += 1;
            coord.x += x_shift;
            coord.y += y_shift;

            if coord == *crossing {
                return Some(steps);
            }
        }
    }

    None
}

pub fn wire_path(wire_instructions: &[Instruction]) -> AdventResult<HashSet<Coord>> {
    let mut path: HashSet<Coord> = HashSet::new();
    let mut coord = Coord { x: 0, y: 0 };

    for instruction in wire_instructions.iter() {
        let (x_shift, y_shift) = match instruction.direction {
            Direction::L => (-1, 0),
            Direction::R => (1, 0),
            Direction::U => (0, 1),
            Direction::D => (0, -1),
        };

        for _ in 0..instruction.length {
            coord.x += x_shift;
            coord.y += y_shift;
            path.insert(coord);
        }
    }

    Ok(path)
}

#[test]
fn test_case_0() {
    let wire1 = parse_instructions(&["R8", "U5", "L5", "D3"]).unwrap();
    let wire2 = parse_instructions(&["U7", "R6", "D4", "L4"]).unwrap();
    assert_eq!(6, closest_crossing(&wire1, &wire2).unwrap());
    assert_eq!(30, cheapest_crossing(&wire1, &wire2).unwrap());
}

#[test]
fn test_case_1() {
    let wire1 = parse_instructions(&["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"])
        .unwrap();
    let wire2 =
        parse_instructions(&["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"]).unwrap();
    assert_eq!(159, closest_crossing(&wire1, &wire2).unwrap());
    assert_eq!(610, cheapest_crossing(&wire1, &wire2).unwrap());
}

#[test]
fn test_case_2() {
    let wire1 = parse_instructions(&[
        "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
    ])
    .unwrap();
    let wire2 = parse_instructions(&[
        "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
    ])
    .unwrap();
    assert_eq!(135, closest_crossing(&wire1, &wire2).unwrap());
    assert_eq!(410, cheapest_crossing(&wire1, &wire2).unwrap());
}
