#![cfg(test)]

use super::*;

#[test]
fn algo_module_fuel_req() {
    assert_eq!(module_fuel_req(14), 2);
    assert_eq!(module_fuel_req(1969), 966);
    assert_eq!(module_fuel_req(100756), 50346);
}

