mod tests;

use std::iter;
use std::str::FromStr;

pub fn module_fuel_req(mass: i32) -> i32 {
    iter::successors(Some(mass / 3 - 2), |&cur| {
        Some(cur / 3 - 2).filter(|&x| x > 0)
    })
    .sum()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AaVec {
    dir: Dir,
    magnitude: i32
}

#[derive(Copy, Clone, Debug)]
pub struct ParseAaVecError;

impl FromStr for AaVec {
    type Err = ParseAaVecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s.chars().next().ok_or(ParseAaVecError)?;
        let dir = match  first {
            'U' => Dir::Up,
            'D' => Dir::Down,
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => return Err(ParseAaVecError)
        };
        let magnitude = s[first.len_utf8()..].parse().or(Err(ParseAaVecError))?;
        Ok(AaVec{ dir, magnitude })
    }
}

pub fn closest_wire_intersection(a: &[AaVec], b: &[AaVec]) -> i32 {
  unimplemented!()
}