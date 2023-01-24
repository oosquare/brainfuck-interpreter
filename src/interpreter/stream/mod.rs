#![allow(unused)]

use std::cell::RefCell;
use std::io::{stdin, BufReader, Read, Stdin};
use std::mem::replace;
use std::rc::Rc;

pub const EOF: i8 = -1;

pub trait InStream {
    fn read(&mut self) -> i8;
}

pub struct NullInStream {}

impl InStream for NullInStream {
    fn read(&mut self) -> i8 {
        EOF
    }
}

pub struct StandardInStream {
    reader: BufReader<Stdin>,
}

impl StandardInStream {
    fn new() -> Self {
        Self {
            reader: BufReader::new(stdin()),
        }
    }
}

impl InStream for StandardInStream {
    fn read(&mut self) -> i8 {
        let mut buf = [0u8; 1];
        let res = self.reader.read(&mut buf);

        match res {
            Ok(0) | Err(_) => EOF,
            _ => buf[0] as i8,
        }
    }
}

pub trait OutStream {
    fn write(&mut self, content: i32);
}

pub struct CharStandardOutStream {}

impl OutStream for CharStandardOutStream {
    fn write(&mut self, content: i32) {
        print!(
            "{}",
            std::primitive::char::from_u32(content as u32).unwrap_or('�')
        );
    }
}

pub struct IntStandardOutStream {}

impl OutStream for IntStandardOutStream {
    fn write(&mut self, content: i32) {
        print!("{} ", content);
    }
}

pub struct VecStandardOutStream {
    output: Rc<RefCell<Vec<i32>>>,
}

impl VecStandardOutStream {
    fn new() -> Self {
        Self { output: Rc::new(RefCell::new(vec![])) }
    }

    fn output(&mut self) -> Rc<RefCell<Vec<i32>>> {
        Rc::clone(&self.output)
    }
}

impl OutStream for VecStandardOutStream {
    fn write(&mut self, content: i32) {
        self.output.borrow_mut().push(content);
    }
}