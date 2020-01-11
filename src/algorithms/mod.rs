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

pub fn iter_digits(mut n: u32, digits: u32) -> impl Iterator<Item = u32> {
    let mut ret = Vec::new();
    for _ in 0..digits {
        ret.push(if n != 0 {
            let d = n % 10;
            n /= 10;
            d
        } else {
            0
        });
    }
    ret.into_iter().rev()
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

    #[test]
    fn algo_test_iter_digits() {
        assert_eq!(
            &[1, 2, 5],
            iter_digits(125, 3).collect::<Vec<_>>().as_slice()
        );
    }

    #[test]
    fn algo_test_iter_digits_leading_zero() {
        assert_eq!(
            &[0, 0, 4, 3, 2],
            iter_digits(432, 5).collect::<Vec<_>>().as_slice()
        );
    }
}
