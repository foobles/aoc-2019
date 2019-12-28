mod tests;
mod intcode;

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
    let mass_file = File::open("data/module_mass.txt")?;
    let mass_reader = BufReader::new(mass_file);
    let total_fuel_req: io::Result<i32> = mass_reader
        .lines()
        .try_fold(0, |acc, line| {
            Ok(acc + module_fuel_req(line?.parse::<i32>().unwrap()))
        });
    println!("Total fuel requirement: {}", total_fuel_req?);

    let intcode_file = File::open("data/intcode_prog.txt")?;
    let intcode_reader = BufReader::new(intcode_file);
    let mut code: Vec<_> = intcode_reader
        .split(b',')
        .map(|x| {
                String::from_utf8(x.unwrap()).unwrap().parse::<i32>().unwrap()
        })
        .collect();

    for noun in 0..code.len() {
        for verb in 0..code.len() {
            code[1] = noun as i32;
            code[2] = verb as i32;
            let mut machine = intcode::Machine::new(code.clone());
            if let Ok(n) = machine.run() {
                if n == 19690720 {
                    println!("Noun={}, Verb={}", noun, verb);
                }
            }
        }
    }
    Ok(())
}
