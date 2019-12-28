#![cfg(test)]

use super::*;

fn test_machine_states(input: &[i32], output: &[i32]) {
    let mut machine = Machine::new(input.to_vec());
    machine.run().unwrap();
    assert_eq!(machine.code.as_slice(), output);
}

#[test]
fn intcode_machine_add() {
    test_machine_states(
        &[1,0,0,0,99],
        &[2,0,0,0,99]
    );
}

#[test]
fn intcode_machine_mul() {
    test_machine_states(
        &[2,3,0,3,99],
        &[2,3,0,6,99]
    );
}

#[test]
fn intcode_machine_modify_after_end() {
    test_machine_states(
        &[2,4,4,5,99,0],
        &[2,4,4,5,99,9801]
    );
}

#[test]
fn intcode_machine_modify_before_and_after() {
    test_machine_states(
        &[1,1,1,4,99,5,6,0,99],
        &[30,1,1,4,2,5,6,0,99]
    );
}