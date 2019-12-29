mod tests;

use std::mem;

pub struct Machine {
    code: Vec<i32>,
    cur: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum Error {
    UnknownOpcode{ opcode: i32 },
    TooFewArguments,
    OutOfBounds
}

macro_rules! access_args {
    ($machine:expr => $(($($def:tt)*))*) => {
        $(
            access_arg!($machine, $($def)*);
        )*
    }
}

macro_rules! access_arg {
    ($machine:expr, let $binding:pat = *arg $idx:expr) => {
        let $binding = $machine.get($machine.get_arg($idx)?)?;
    };
    ($machine:expr, let $binding:pat = arg $idx:expr) => {
        let $binding = $machine.get_arg($idx)?;
    }
}

const OP_ADD: i32 = 1;
const OP_MUL: i32 = 2;
const OP_END: i32 = 99;

impl Machine {
    pub fn new(code: Vec<i32>) -> Self {
        Machine { code, cur: 0 }
    }

    pub fn run(&mut self) -> Result<i32, Error> {
        while self.cur < self.code.len() {
            //println!("CUR={:02} | {:?}", self.cur, self.code);
            match self.code[self.cur] {
                OP_ADD => self.add()?,
                OP_MUL => self.mul()?,
                OP_END => break,
                n => return Err(Error::UnknownOpcode{ opcode: n })
            }
        }
        Ok(self.code[0])
    }

    fn add(&mut self) -> Result<(), Error> {
        access_args!{self =>
            (let a = *arg 0)
            (let b = *arg 1)
            (let r_addr = arg 2)
        }
        self.set(r_addr, a + b)?;
        self.cur += 4;
        Ok(())
    }

    fn mul(&mut self) -> Result<(), Error> {
        access_args!{self =>
            (let a = *arg 0)
            (let b = *arg 1)
            (let r_addr = arg 2)
        }
        self.set(r_addr, a * b)?;
        self.cur += 4;
        Ok(())
    }

    fn set(&mut self, idx: i32, val: i32) -> Result<i32, Error> {
        if idx < 0 {
            return Err(Error::OutOfBounds);
        }
        self.code
            .get_mut(idx as usize)
            .map(|x| mem::replace(x, val))
            .ok_or(Error::OutOfBounds)
    }

    fn get(&self, idx: i32) -> Result<i32, Error> {
        if idx < 0 {
            return Err(Error::OutOfBounds);
        }
        self.code.get(idx as usize).copied().ok_or(Error::OutOfBounds)
    }

    fn get_arg(&self, idx: usize) -> Result<i32, Error> {
        self.get((self.cur + idx + 1) as i32)
    }
}
