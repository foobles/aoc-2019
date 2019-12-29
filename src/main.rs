mod intcode;
mod algorithms;

use std::fs::File;
use std::io::{self, BufReader, prelude::*};


fn main() -> io::Result<()> {
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
