mod tests;

use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn into_coords(self) -> (i32, i32) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0)
        }
    }
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
        let magnitude = s[1..].parse().or(Err(ParseAaVecError))?;
        Ok(AaVec{ dir, magnitude })
    }
}

pub struct Wire {
    path: Vec<AaVec>,
}

impl Wire {
    fn for_each_point<F: FnMut((i32, i32))>(&self, mut op: F) {
        let (mut cx, mut cy) = (0, 0);
        for &v in &self.path {
            let (dx, dy) = v.dir.into_coords();
            for i in 0..v.magnitude {
                cx += dx;
                cy += dy;
                op((cx, cy));
            }
        }
    }
}

pub fn closest_wire_intersection(wire_a: &Wire, wire_b: &Wire) -> Option<i32> {
    let mut points = HashSet::new();
    let (mut cx, mut cy) = (0, 0);
    wire_a.for_each_point(|p| {
        points.insert(p);
    });
    let mut min_distance: Option<i32> = None;
    wire_b.for_each_point(|p| {
        if points.contains(&p) {
            let cur_distance = p.0.abs() + p.1.abs();
            min_distance = min_distance
                .map(|m| m.min(cur_distance))
                .or(Some(cur_distance));
        }
    });
    min_distance
}