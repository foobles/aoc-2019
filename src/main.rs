mod intcode;

#[allow(dead_code)]
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

fn main() -> io::Result<()> {
    let mut machine = parse_intcode(BufReader::new(File::open("data/ac_prog.txt")?))?;
    let mut input = BufReader::new(io::stdin());
    let mut output = io::stdout();
    println!("{}", machine.run(&mut input, &mut output).unwrap());
    Ok(())
}
