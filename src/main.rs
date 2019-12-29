mod intcode;
mod algorithms;

use std::fs::File;
use std::io::{self, BufReader, prelude::*};

use algorithms::wires;

fn main() -> io::Result<()> {
    let wire_file = File::open("data/wires.txt")?;
    let wire_reader = BufReader::new(wire_file);
    let mut wire_line_iter = wire_reader.lines();
    let wire_1 = wire_line_iter.next().unwrap()?.parse().unwrap();
    let wire_2 = wire_line_iter.next().unwrap()?.parse().unwrap();

    println!("{:?}", wires::closest_wire_intersection(&wire_1, &wire_2));

    Ok(())
}
