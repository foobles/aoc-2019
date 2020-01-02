mod tests;

use std::mem;
use std::io::prelude::*;

pub struct Machine {
    code: Vec<i32>,
    cur: usize,
}

#[derive(Debug)]
pub enum Error {
    UnknownOpcode{ opcode: i32 },
    UnknownOpmode{ mode: i32},
    OutOfBounds,
    Eof,
    InvalidInput,
    IoError(std::io::Error)
}

macro_rules! access_args {
    ($machine:expr => $(($($def:tt)*))*) => {
        $(
            access_arg!($machine, $($def)*);
        )*
    }
}

macro_rules! access_arg {
    ($machine:expr, let $binding:pat = arg $idx:expr) => {
        let $binding = $machine.get_arg($idx)?;
    };
    ($machine:expr, let $binding:pat = rawarg $idx:expr) => {
        let $binding = $machine.get_arg_raw($idx)?;
    }
}

const OP_ADD: i32 = 1;
const OP_MUL: i32 = 2;
const OP_IN:  i32 = 3;
const OP_OUT: i32 = 4;
const OP_JT:  i32 = 5;
const OP_JF:  i32 = 6;
const OP_LT:  i32 = 7;
const OP_EQ:  i32 = 8;
const OP_END: i32 = 99;

impl Machine {
    pub fn new(code: Vec<i32>) -> Self {
        Machine { code, cur: 0 }
    }

    pub fn run<R, W>(&mut self, reader: &mut R, writer: &mut W) -> Result<i32, Error>
    where
        R: BufRead,
        W: Write
    {
        while self.cur < self.code.len() {
            //println!("CUR={:02} | {:?}", self.cur, self.code);
            match self.code[self.cur] % 100 {
                OP_ADD => self.bin_op(|x, y| x + y),
                OP_MUL => self.bin_op(|x, y| x * y),
                OP_IN => self.store_input(reader),
                OP_OUT => self.output(writer),
                OP_JT => self.jump_if(|x| x != 0),
                OP_JF => self.jump_if(|x| x == 0),
                OP_LT=> self.compare(|x, y| x < y),
                OP_EQ => self.compare(|x, y| x == y),
                OP_END => break,
                n => return Err(Error::UnknownOpcode{ opcode: n })
            }?;
        }
        Ok(self.code[0])
    }

    fn bin_op<F: FnOnce(i32, i32) -> i32>(&mut self, op: F) -> Result<(), Error> {
        access_args!{self =>
            (let a = arg 0)
            (let b = arg 1)
            (let r_addr = rawarg 2)
        }
        self.set(r_addr, op(a, b))?;
        self.cur += 4;
        Ok(())
    }

    fn store_input<R: BufRead>(&mut self, reader: &mut R) -> Result<(), Error> {
        let mut input = String::new();
        if reader.read_line( &mut input).map_err(Error::IoError)? == 0{
            return Err(Error::Eof);
        }
        access_args!{self =>
            (let r_addr = rawarg 0)
        }
        self.set(r_addr, input.trim().parse().or(Err(Error::InvalidInput))?)?;
        self.cur += 2;
        Ok(())
    }

    fn output<W: Write>(&mut self, writer: &mut W) -> Result<(), Error> {
        access_args!{self =>
            (let val = arg 0)
        }
        write!(writer, ": {}\n", val).map_err(Error::IoError)?;
        self.cur += 2;
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

    fn jump_if<F: FnOnce(i32) -> bool>(&mut self, cond: F) -> Result<(), Error> {
        access_args!{self =>
            (let val = arg 0)
            (let dest = arg 1)
        }
        if cond(val) {
            self.jump(dest)?;
        } else {
            self.cur += 3;
        }
        Ok(())
    }

    fn compare<F: FnOnce(i32, i32) -> bool>(&mut self, comp: F) -> Result<(), Error> {
        self.bin_op(|x, y| if comp(x, y) { 1 } else { 0 })
    }

    fn jump(&mut self, loc: i32) -> Result<(), Error> {
        if loc < 0 || loc as usize >= self.code.len() {
            Err(Error::OutOfBounds)
        } else {
            self.cur = loc as usize;
            Ok(())
        }
    }

    fn get(&self, idx: i32) -> Result<i32, Error> {
        if idx < 0 {
            return Err(Error::OutOfBounds);
        }
        self.code.get(idx as usize).copied().ok_or(Error::OutOfBounds)
    }

    fn get_arg_raw(&self, idx: usize) -> Result<i32, Error> {
        self.get((self.cur + idx + 1) as i32)
    }

    fn get_arg_mode(&self, idx: usize) -> i32 {
        self.code[self.cur] / 10_i32.pow(2 + idx as u32) % 10
    }

    fn get_arg(&self, idx: usize) -> Result<i32, Error> {
        let raw = self.get_arg_raw(idx);
        match self.get_arg_mode(idx) {
            0 => self.get(raw?),
            1 => raw,
            n => Err(Error::UnknownOpmode {mode: n})
        }
    }
}

