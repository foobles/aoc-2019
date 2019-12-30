mod tests;
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
    let mut on_dup = false;
    let mut digits = 1;
    let mut prev = pw % 10;
    pw /= 10;
    while pw > 0 {
        digits += 1;
        let cur = pw % 10;
        if prev == cur {
            dup_found = true;
            if on_dup {
                return false;
            }
            on_dup = true;
        } else {
            on_dup = false;
            if prev < cur {
                return false;
            }
        }
        prev = cur;
        pw /= 10;
    }
    dup_found && digits == 6
}

pub fn password_count(lower: i32, upper: i32) -> usize {
    (lower..=upper).filter(|&x| is_valid_password(x)).count()
}