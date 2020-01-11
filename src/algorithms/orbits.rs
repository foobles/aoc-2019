use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter;

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
    orbits
        .iter()
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
    let cca_dist = x_path
        .into_iter()
        .rev()
        .zip(y_path.into_iter().rev())
        .take_while(|&(a, b)| a == b)
        .count()
        - 1;
    x_dist + y_dist - 2 * cca_dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algo_test_count_orbits() {
        assert_eq!(
            42,
            count_orbits(
                &[
                    ("COM", "B"),
                    ("B", "C"),
                    ("C", "D"),
                    ("D", "E"),
                    ("E", "F"),
                    ("B", "G"),
                    ("G", "H"),
                    ("D", "I"),
                    ("E", "J"),
                    ("J", "K"),
                    ("K", "L")
                ]
                .iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect::<Vec<_>>()
            )
        );
    }

    #[test]
    fn algo_test_tree_distance() {
        let map = [
            ("COM", "B"),
            ("B", "C"),
            ("C", "D"),
            ("D", "E"),
            ("E", "F"),
            ("B", "G"),
            ("G", "H"),
            ("D", "I"),
            ("E", "J"),
            ("J", "K"),
            ("K", "L"),
        ]
        .iter()
        .map(|&(a, b)| (b.to_string(), a))
        .collect::<HashMap<_, _>>();
        assert_eq!(4, tree_distance(&map, "K", "I"));
    }
}
