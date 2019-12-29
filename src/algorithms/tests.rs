#![cfg(test)]

use super::*;

#[test]
fn algo_module_fuel_req() {
    assert_eq!(module_fuel_req(14), 2);
    assert_eq!(module_fuel_req(1969), 966);
    assert_eq!(module_fuel_req(100756), 50346);
}


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
