mod tests;
pub mod wires;

use std::iter;
use std::collections::HashMap;

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

pub fn count_orbits(orbits: &[(String, String)]) -> usize {
    let map: HashMap<String, &str> = orbits
        .iter()
        .map(|(x, y)| (y.clone(), x.as_str()))
        .collect();
    let mut count = 0;
    for (_, cur) in orbits {
        let mut cur = cur.as_str();
        while let Some(next) = map.get(cur) {
            cur = *next;
            count += 1;
        }
    }
    count
}