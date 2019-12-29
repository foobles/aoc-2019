#![cfg(test)]

use super::*;

#[test]
fn algo_parse_aa_vec() {
    assert_eq!("U432".parse::<AaVec>().unwrap(), AaVec{ dir: Dir::Up, magnitude: 432});
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
        closest_wire_intersection(
            &w1.parse().unwrap(),
            &w2.parse().unwrap()
        )
    );
}

#[test]
fn wires_intersect() {
    test_intersection(
        Some(159),
        "R75,D30,R83,U83,L12,D49,R71,U7,L72",
        "U62,R66,U55,R34,D71,R55,D58,R83"
    );
}

#[test]
fn wires_not_intersect() {
    test_intersection(
        None,
        "R75,D30",
        "U62,R66"
    );
}
