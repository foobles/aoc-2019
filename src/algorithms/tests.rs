#![cfg(test)]

use super::*;

#[test]
fn algo_module_fuel_req() {
    assert_eq!(module_fuel_req(14), 2);
    assert_eq!(module_fuel_req(1969), 966);
    assert_eq!(module_fuel_req(100756), 50346);
}

#[test]
fn algo_test_password() {
    assert!(is_valid_password(122345));
}

#[test]
fn algo_test_password_wrong_digits() {
    assert!(!is_valid_password(122));
}

#[test]
fn algo_test_password_not_increasing() {
    assert!(!is_valid_password(122340));
}

#[test]
fn algo_test_password_no_double() {
    assert!(!is_valid_password(123456));
}

#[test]
fn algo_test_password_excess_double() {
    assert!(!is_valid_password(122234));
}