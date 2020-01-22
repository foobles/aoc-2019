#![cfg(test)]

use super::*;
use std::iter;

fn test_machine_states(input: &[i32], expected_output: &[i32]) {
    let mut machine = Machine::new(input.to_vec());
    machine.run_to_end(std::iter::empty()).unwrap();
    assert_eq!(machine.code.as_slice(), expected_output);
}

fn test_machine_output(machine: &[i32], input: &[i32], expected_output: &[i32]) {
    let mut machine = Machine::new(machine.to_vec());
    assert_eq!(
        machine
            .run_to_end(input.iter().copied())
            .unwrap()
            .as_slice(),
        expected_output
    );
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

#[test]
fn intcode_machine_run_twice() {
    let mut machine = Machine::new(vec![3, 0, 3, 1, 1, 0, 1, 0, 4, 0, 99]);
    let mut output = Vec::new();
    machine.run_with(iter::once(5), &mut output).unwrap();
    machine.run_with(iter::once(3), &mut output).unwrap();
    assert!(machine.done());
    assert_eq!(output[0], 8);
}

#[test]
fn intcode_machine_zero_relative_base() {
    test_machine_states(&[1201, 2, 100, 0, 99], &[200, 2, 100, 0, 99]);
}

#[test]
fn intcode_machine_pos_relative_base() {
    test_machine_states(
        &[109, 2, 2202, -2, 1, 0, 99],
        &[-218, 2, 2202, -2, 1, 0, 99],
    );
}

#[test]
fn intcode_machine_neg_relative_base() {
    test_machine_states(&[109, -3, 20001, 1, 3, 5, 99], &[109, -3, -2, 1, 3, 5, 99]);
}

#[test]
fn intcode_machine_invalid_opcode() {
    assert!(Machine::new(vec![11101, 1, 2, 3, 99])
        .run_to_end(iter::empty())
        .is_err())
}
