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