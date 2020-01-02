mod intcode;
mod algorithms;

use std::fs::File;
use std::io::{self, BufReader, prelude::*};

use intcode::Machine;

use algorithms::wires;

fn parse_intcode<B: BufRead>(file: B) -> io::Result<Machine> {
    file.split(b',')
        .map(|x| {
            Ok(String::from_utf8(x?).unwrap().parse().unwrap())
        })
        .collect::<Result<_, _>>()
        .map(Machine::new)
}

fn main() -> io::Result<()> {
    println!("Possible passwords = {}", algorithms::password_count(235741, 706948));
    let mut machine = parse_intcode(BufReader::new(File::open("data/intcode_prog.txt")?))?;
    println!("{}", machine.run().unwrap());
    Ok(())
}
