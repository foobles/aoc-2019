pub mod amplifier;
pub mod orbits;
pub mod wires;

use std::iter;

pub fn module_fuel_req(mass: i32) -> i32 {
    iter::successors(Some(mass / 3 - 2), |&cur| {
        Some(cur / 3 - 2).filter(|&x| x > 0)
    })
    .sum()
}

fn is_valid_password(mut pw: i32) -> bool {
    let mut dup_found = false;
    let mut dup_count = 0;
    let mut digits = 1;
    let mut prev = pw % 10;
    pw /= 10;
    while pw > 0 {
        digits += 1;
        let cur = pw % 10;
        if prev == cur {
            dup_count += 1;
        } else {
            if prev < cur {
                return false;
            }
            if dup_count == 1 {
                dup_found = true;
            }
            dup_count = 0;
        }
        prev = cur;
        pw /= 10;
    }
    (dup_found || dup_count == 1) && digits == 6
}

pub fn password_count(lower: i32, upper: i32) -> usize {
    (lower..=upper).filter(|&x| is_valid_password(x)).count()
}

fn permutation_swaps(len: usize) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    let mut c = vec![0; len];
    let mut i = 0;
    while i < len {
        if c[i] < i {
            ret.push(if i % 2 == 0 { (0, i) } else { (c[i], i) });
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }
    ret
}

pub fn permutations(mut vals: Vec<i32>) -> impl Iterator<Item = Vec<i32>> {
    iter::once(vals.clone()).chain(
        permutation_swaps(vals.len())
            .into_iter()
            .map(move |(i, j)| {
                vals.swap(i, j);
                vals.clone()
            }),
    )
}

#[cfg(test)]
mod tests {
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
    fn algo_test_password_start_double() {
        assert!(is_valid_password(112345));
    }

    #[test]
    fn algo_test_password_with_triple() {
        assert!(is_valid_password(112233));
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
}
