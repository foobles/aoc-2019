mod tests;

use std::iter;
use std::fs::File;
use std::io::{self, BufReader, prelude::*};

fn module_fuel_req(mass: i32) -> i32 {
    iter::successors(Some(mass / 3 - 2), |&cur| {
        Some(cur / 3 - 2).filter(|&x| x > 0)
    })
    .sum()
}

fn main() -> io::Result<()> {
    let file = File::open("data/module_mass.txt")?;
    let reader = BufReader::new(file);
    let total_fuel_req: io::Result<i32> = reader
        .lines()
        .try_fold(0, |acc, line| {
            Ok(acc + module_fuel_req(line?.parse::<i32>().unwrap()))
        });

    println!("Total fuel requirement: {}", total_fuel_req?);
    Ok(())
}
