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
fn algo_wire_intersection() {

}