#![allow(dead_code)]

mod algorithms;
mod intcode;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use intcode::Machine;

fn parse_intcode<B: BufRead>(file: B) -> io::Result<Machine> {
    file.split(b',')
        .map(|x| Ok(String::from_utf8(x?).unwrap().parse().unwrap()))
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
    //    let orbits = parse_orbits(BufReader::new(File::open("data/orbits.txt")?))?;
    //    let dist = algorithms::orbit_distance(&orbits, "YOU", "SAN");
    //    println!("{}", dist);
    //    Ok(())

    let machine = parse_intcode(BufReader::new(File::open("data/amp_prog.txt")?))?;

    let max = algorithms::permutations(vec![0, 1, 2, 3, 4])
        .map(|v| {
            let r = algorithms::amplifier::score_setting(&machine, v)
                .unwrap()
                .unwrap();
            r
        })
        .max()
        .unwrap();

    println!("{:?}", max);

    Ok(())
}
