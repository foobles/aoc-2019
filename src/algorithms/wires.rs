use std::collections::HashMap;
use std::str::FromStr;

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
            Dir::Right => (1, 0),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AaVec {
    dir: Dir,
    magnitude: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct ParseAaVecError;

impl FromStr for AaVec {
    type Err = ParseAaVecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s.chars().next().ok_or(ParseAaVecError)?;
        let dir = match first {
            'U' => Dir::Up,
            'D' => Dir::Down,
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => return Err(ParseAaVecError),
        };
        let magnitude = s[1..].parse().or(Err(ParseAaVecError))?;
        Ok(AaVec { dir, magnitude })
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
            for _ in 0..v.magnitude {
                cx += dx;
                cy += dy;
                op((cx, cy));
            }
        }
    }
}

pub fn closest_wire_intersection(wire_a: &Wire, wire_b: &Wire) -> Option<i32> {
    let mut points = HashMap::new();
    let mut cur_travelled = 0;
    wire_a.for_each_point(|p| {
        cur_travelled += 1;
        points.insert(p, cur_travelled);
    });
    let mut min_distance: Option<i32> = None;
    cur_travelled = 0;
    wire_b.for_each_point(|p| {
        cur_travelled += 1;
        if let Some(&dist) = points.get(&p) {
            let cur_distance = cur_travelled + dist;
            min_distance = min_distance
                .map(|m| m.min(cur_distance))
                .or(Some(cur_distance));
        }
    });
    min_distance
}

#[derive(Copy, Clone, Debug)]
pub struct ParseWireError;

impl FromStr for Wire {
    type Err = ParseWireError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(|x| x.parse())
            .collect::<Result<_, _>>()
            .map(|path| Wire { path })
            .or(Err(ParseWireError))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algo_parse_aa_vec() {
        assert_eq!(
            "U432".parse::<AaVec>().unwrap(),
            AaVec {
                dir: Dir::Up,
                magnitude: 432
            }
        );
    }

    #[test]
    #[should_panic]
    fn algo_parse_aa_vec_no_dir() {
        "123".parse::<AaVec>().unwrap();
    }

    #[test]
    #[should_panic]
    fn algo_parse_aa_vec_no_magnitude() {
        "D".parse::<AaVec>().unwrap();
    }

    #[test]
    fn parse_wire() {
        assert_eq!(
            "L32,U20,R16,D50".parse::<Wire>().unwrap().path,
            ["L32", "U20", "R16", "D50"]
                .iter()
                .map(|&x| x.parse::<AaVec>().unwrap())
                .collect::<Vec<_>>()
        );
    }

    fn test_intersection(dist: Option<i32>, w1: &str, w2: &str) {
        assert_eq!(
            dist,
            closest_wire_intersection(&w1.parse().unwrap(), &w2.parse().unwrap())
        );
    }

    #[test]
    fn wires_intersect() {
        test_intersection(
            Some(610),
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
        );
    }

    #[test]
    fn wires_not_intersect() {
        test_intersection(None, "R75,D30", "U62,R66");
    }
}
