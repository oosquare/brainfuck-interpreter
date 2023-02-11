pub mod config;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::{stdin, BufReader, Read, Stdin};
use std::rc::Rc;

use config::{Config, Input, Output};

pub const EOF: i8 = -1;

pub trait InStream {
    fn read(&mut self) -> i8;
}

pub struct NullInStream;

impl InStream for NullInStream {
    fn read(&mut self) -> i8 {
        EOF
    }
}

pub struct StandardInStream {
    reader: BufReader<Stdin>,
}

impl StandardInStream {
    pub fn new() -> Self {
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

pub struct VecInStream {
    input: Rc<RefCell<VecDeque<i8>>>,
}

impl VecInStream {
    pub fn new(input: Rc<RefCell<VecDeque<i8>>>) -> Self {
        Self { input }
    }
}

impl InStream for VecInStream {
    fn read(&mut self) -> i8 {
        self.input.borrow_mut().pop_front().unwrap_or(EOF)
    }
}

pub trait OutStream {
    fn write(&mut self, content: i32);
}

pub struct NullOutStream;

impl OutStream for NullOutStream {
    fn write(&mut self, _content: i32) {}
}

pub struct CharStandardOutStream;

impl OutStream for CharStandardOutStream {
    fn write(&mut self, content: i32) {
        print!(
            "{}",
            std::primitive::char::from_u32(content as u32).unwrap_or('�')
        );
    }
}

pub struct IntStandardOutStream;

impl OutStream for IntStandardOutStream {
    fn write(&mut self, content: i32) {
        print!("{content} ");
    }
}

pub struct VecOutStream {
    output: Rc<RefCell<VecDeque<i32>>>,
}

impl VecOutStream {
    pub fn new(output: Rc<RefCell<VecDeque<i32>>>) -> Self {
        Self { output }
    }
}

impl OutStream for VecOutStream {
    fn write(&mut self, content: i32) {
        self.output.borrow_mut().push_back(content);
    }
}

pub struct Builder {
    input: Input,
    output: Output,
}

#[allow(dead_code)]
impl Builder {
    pub fn new() -> Self {
        Self {
            input: Input::Standard,
            output: Output::CharStandard,
        }
    }

    pub fn with_config(config: Config) -> Self {
        let Config { input, output } = config;
        Self { input, output }
    }

    pub fn input(mut self, input: Input) -> Self {
        self.input = input;
        self
    }

    pub fn output(mut self, output: Output) -> Self {
        self.output = output;
        self
    }

    pub fn build(self) -> (Box<dyn InStream>, Box<dyn OutStream>) {
        let input: Box<dyn InStream> = match self.input {
            Input::Null => Box::new(NullInStream),
            Input::Standard => Box::new(StandardInStream::new()),
            Input::Vec(v) => Box::new(VecInStream::new(v)),
        };

        let output: Box<dyn OutStream> = match self.output {
            Output::Null => Box::new(NullOutStream),
            Output::CharStandard => Box::new(CharStandardOutStream),
            Output::IntStandard => Box::new(IntStandardOutStream),
            Output::Vec(v) => Box::new(VecOutStream::new(v)),
        };

        (input, output)
    }
}
