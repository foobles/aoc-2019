#![cfg(test)]

use super::*;

fn test_machine_states(input: &[i32], expected_output: &[i32]) {
    let mut machine = Machine::new(input.to_vec());
    machine.run(std::iter::empty(), &mut Vec::new()).unwrap();
    assert_eq!(machine.code.as_slice(), expected_output);
}

fn test_machine_output(machine: &[i32], input: &[i32], expected_output: &[i32]) {
    let mut machine = Machine::new(machine.to_vec());
    let mut machine_output = Vec::new();
    machine
        .run(input.iter().copied(), &mut machine_output)
        .unwrap();
    assert_eq!(machine_output.as_slice(), expected_output);
}

#[test]
fn intcode_machine_add() {
    test_machine_states(&[1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]);
}

#[test]
fn intcode_machine_mul() {
    test_machine_states(&[2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]);
}

#[test]
fn intcode_machine_modify_after_end() {
    test_machine_states(&[2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]);
}

#[test]
fn intcode_machine_modify_before() {
    test_machine_states(
        &[1, 1, 1, 4, 99, 5, 6, 0, 99],
        &[30, 1, 1, 4, 2, 5, 6, 0, 99],
    );
}

#[test]
fn intcode_machine_rhs_immediate_mul() {
    test_machine_states(&[1002, 4, 3, 4, 33], &[1002, 4, 3, 4, 99]);
}

#[test]
fn intcode_machine_lhs_immediate_add() {
    test_machine_states(&[101, 50, 4, 4, 49], &[101, 50, 4, 4, 99]);
}

#[test]
fn intcode_machine_cat() {
    test_machine_output(&[3, 0, 4, 0, 99], &[123], &[123]);
}

#[test]
fn intcode_machine_jt() {
    test_machine_output(&[1105, 0, 5, 104, 999, 99], &[], &[999]);
}

#[test]
fn intcode_machine_jf() {
    test_machine_output(&[6, 6, 7, 104, 999, 99, 0, 5], &[], &[]);
}

#[test]
fn intcode_machine_lt_less() {
    test_machine_states(&[1107, 99, 100, 0, 99], &[1, 99, 100, 0, 99]);
}

#[test]
fn intcode_machine_lt_equal() {
    test_machine_states(&[7, 4, 4, 0, 99], &[0, 4, 4, 0, 99]);
}

#[test]
fn intcode_machine_lt_greater() {
    test_machine_states(&[1007, 0, 6, 0, 99], &[0, 0, 6, 0, 99]);
}

#[test]
fn intcode_machine_eq_equal() {
    test_machine_states(&[108, 2, 5, 1, 99, 2], &[108, 1, 5, 1, 99, 2]);
}

#[test]
fn intcode_machine_eq_not_equal() {
    test_machine_states(&[1108, 5, 6, 2, 99], &[1108, 5, 0, 2, 99]);
}
