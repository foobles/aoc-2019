mod intcode;
mod algorithms;

use std::fs::File;
use std::io::{self, BufReader, prelude::*};

use algorithms::wires;

fn main() -> io::Result<()> {
    println!("Possible passwords = {}", algorithms::password_count(235741, 706948));

    Ok(())
}
