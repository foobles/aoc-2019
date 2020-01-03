#![allow(dead_code)]

mod intcode;
mod algorithms;

use std::fs::File;
use std::io::{self, BufReader, prelude::*};

use intcode::Machine;

fn parse_intcode<B: BufRead>(file: B) -> io::Result<Machine> {
    file.split(b',')
        .map(|x| {
            Ok(String::from_utf8(x?).unwrap().parse().unwrap())
        })
        .collect::<Result<_, _>>()
        .map(Machine::new)
}

fn parse_orbits<B: BufRead>(file: B) -> io::Result<Vec<(String, String)>> {
    file.lines()
        .map(|x| {
            let mut x = x?;
            let snd = String::from(&x[4..]);
            x.truncate(3);
            Ok((x, snd))
        })
        .collect()
}

fn main() -> io::Result<()> {
    let orbits = parse_orbits(BufReader::new(File::open("data/orbits.txt")?))?;
    let dist = algorithms::orbit_distance(&orbits, "YOU", "SAN");
    println!("{}", dist);
    Ok(())
}
