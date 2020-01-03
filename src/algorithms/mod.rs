mod tests;
pub mod wires;

use std::iter;
use std::collections::HashMap;
use std::borrow::Borrow;
use std::hash::Hash;

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

pub fn path_to_root<'a, K, V>(tree: &'a HashMap<K, &V>, val: &V) -> impl Iterator<Item = &'a V>
where
    K: Hash + Eq + Borrow<V>,
    V: Hash + Eq + ?Sized,
{
    let mut cur = tree.get(val).copied();
    iter::from_fn(move || {
        let ret = cur.take();
        if let Some(x) = ret {
            cur = tree.get(x).copied();
        }
        ret
    })
}

fn orbit_pairs_to_map(orbits: &[(String, String)]) -> HashMap<String, &str> {
    orbits
        .iter()
        .map(|(x, y)| (y.clone(), x.as_str()))
        .collect()
}

pub fn count_orbits(orbits: &[(String, String)]) -> usize {
    let map = orbit_pairs_to_map(orbits);
    orbits.iter()
        .map(|(_, x)| path_to_root(&map, x).count())
        .sum()
}

pub fn orbit_distance(orbits: &[(String, String)], x: &str, y: &str) -> usize {
    let map = orbit_pairs_to_map(orbits);
    tree_distance(&map, &map[x], &map[y])
}

pub fn tree_distance(tree: &HashMap<String, &str>, x: &str, y: &str) -> usize {
    let x_path: Vec<_> = path_to_root(tree, x).collect();
    let y_path: Vec<_> = path_to_root(tree, y).collect();
    let x_dist = x_path.len();
    let y_dist = y_path.len();
    let cca_dist= x_path
        .into_iter()
        .rev()
        .zip(y_path.into_iter().rev())
        .take_while(|&(a, b)| a == b)
        .count() - 1;
    x_dist + y_dist - 2 * cca_dist
}