mod tests;

use std::mem;

#[derive(Debug, Clone)]
pub struct Machine {
    code: Vec<i64>,
    cur: usize,
    relative_base: i64,
    done: bool,
}

#[derive(Debug)]
pub enum Error {
    UnknownOpcode { opcode: i64 },
    UnknownOpmode { mode: i64 },
    InvalidOpmode { mode: i64 },
    OutOfBounds,
    Eof,
    NoTermination,
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
        let $binding = $machine.get_val_arg($idx)?;
    };
    ($machine:expr, let $binding:pat = addr_arg $idx:expr) => {
        let $binding = $machine.get_addr_arg($idx)?;
    };
}

const OP_ADD: i64 = 1;
const OP_MUL: i64 = 2;
const OP_IN: i64 = 3;
const OP_OUT: i64 = 4;
const OP_JT: i64 = 5;
const OP_JF: i64 = 6;
const OP_LT: i64 = 7;
const OP_EQ: i64 = 8;
const OP_MRB: i64 = 9;
const OP_END: i64 = 99;

impl Machine {
    pub fn new(code: Vec<i64>) -> Self {
        Machine {
            code,
            cur: 0,
            relative_base: 0,
            done: false,
        }
    }

    pub fn with_initial_size(mut code: Vec<i64>, size: usize) -> Self {
        code.resize(size, 0);
        Self::new(code)
    }

    pub fn run_to_end<I>(&mut self, input: I) -> Result<Vec<i64>, Error>
    where
        I: IntoIterator<Item = i64>,
    {
        let mut output = Vec::new();
        self.cur = 0;
        self.run_inner_loop(input.into_iter(), &mut output)?;
        Ok(output)
    }

    pub fn run_with<I>(&mut self, input: I, output: &mut Vec<i64>) -> Result<usize, Error>
    where
        I: IntoIterator<Item = i64>,
    {
        let output_init_len = output.len();
        match self.run_inner_loop(input.into_iter(), output) {
            Ok(_) | Err(Error::Eof) => Ok(output.len() - output_init_len),
            Err(e) => Err(e),
        }
    }

    pub fn done(&self) -> bool {
        self.done
    }

    fn run_inner_loop(
        &mut self,
        mut input: impl Iterator<Item = i64>,
        output: &mut Vec<i64>,
    ) -> Result<(), Error> {
        while !self.done {
            self.cur = match self.code[self.cur] % 100 {
                OP_ADD => self.bin_op(|x, y| x + y),
                OP_MUL => self.bin_op(|x, y| x * y),
                OP_IN => self.store_input(&mut input),
                OP_OUT => self.output(output),
                OP_JT => self.jump_if(|x| x != 0),
                OP_JF => self.jump_if(|x| x == 0),
                OP_LT => self.compare(|x, y| x < y),
                OP_EQ => self.compare(|x, y| x == y),
                OP_MRB => self.modify_relative_base(),
                OP_END => {
                    self.done = true;
                    Ok(self.cur)
                }
                n => Err(Error::UnknownOpcode { opcode: n }),
            }?;
        }
        Ok(())
    }

    fn bin_op<F: FnOnce(i64, i64) -> i64>(&mut self, op: F) -> Result<usize, Error> {
        access_args! {self =>
            (let a = arg 0)
            (let b = arg 1)
            (let r_addr = addr_arg 2)
        }
        self.set(r_addr, op(a, b))?;
        self.inc(4)
    }

    fn store_input<I>(&mut self, input: &mut I) -> Result<usize, Error>
    where
        I: Iterator<Item = i64>,
    {
        access_args! {self =>
            (let r_addr = addr_arg 0)
        }
        self.set(r_addr, input.next().ok_or(Error::Eof)?)?;
        self.inc(2)
    }

    fn output(&self, out: &mut Vec<i64>) -> Result<usize, Error> {
        access_args! {self =>
            (let val = arg 0)
        }
        out.push(val);
        self.inc(2)
    }

    fn set(&mut self, idx: i64, val: i64) -> Result<i64, Error> {
        if idx < 0 {
            return Err(Error::OutOfBounds);
        }
        self.code
            .get_mut(idx as usize)
            .map(|x| mem::replace(x, val))
            .ok_or(Error::OutOfBounds)
    }

    fn jump_if<F: FnOnce(i64) -> bool>(&self, cond: F) -> Result<usize, Error> {
        access_args! {self =>
            (let val = arg 0)
            (let dest = arg 1)
        }
        if cond(val) {
            self.jump(dest)
        } else {
            self.inc(3)
        }
    }

    fn modify_relative_base(&mut self) -> Result<usize, Error> {
        access_args! {self =>
            (let delta = arg 0)
        }
        self.relative_base += delta;
        self.inc(2)
    }

    fn compare<F: FnOnce(i64, i64) -> bool>(&mut self, comp: F) -> Result<usize, Error> {
        self.bin_op(|x, y| if comp(x, y) { 1 } else { 0 })
    }

    fn jump(&self, loc: i64) -> Result<usize, Error> {
        if loc < 0 || loc as usize >= self.code.len() {
            Err(Error::OutOfBounds)
        } else {
            Ok(loc as usize)
        }
    }

    fn inc(&self, amount: usize) -> Result<usize, Error> {
        if self.cur + amount >= self.code.len() {
            Err(Error::NoTermination)
        } else {
            Ok(self.cur + amount)
        }
    }

    fn get(&self, idx: i64) -> Result<i64, Error> {
        if idx < 0 {
            return Err(Error::OutOfBounds);
        }
        self.code
            .get(idx as usize)
            .copied()
            .ok_or(Error::OutOfBounds)
    }

    fn get_arg_raw(&self, idx: usize) -> Result<i64, Error> {
        self.get((self.cur + idx + 1) as i64)
    }

    fn get_arg_mode(&self, idx: usize) -> i64 {
        self.code[self.cur] / 10_i64.pow(2 + idx as u32) % 10
    }

    fn get_val_arg(&self, idx: usize) -> Result<i64, Error> {
        let raw = self.get_arg_raw(idx);
        match self.get_arg_mode(idx) {
            0 => self.get(raw?),
            1 => raw,
            2 => self.get(raw? + self.relative_base),
            n => Err(Error::UnknownOpmode { mode: n }),
        }
    }

    fn get_addr_arg(&self, idx: usize) -> Result<i64, Error> {
        let raw = self.get_arg_raw(idx);
        match self.get_arg_mode(idx) {
            0 => raw,
            1 => Err(Error::InvalidOpmode { mode: 1 }),
            2 => Ok(raw? + self.relative_base),
            mode => Err(Error::UnknownOpmode { mode }),
        }
    }
}
