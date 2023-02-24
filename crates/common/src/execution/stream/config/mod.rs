use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Clone)]
pub struct Config {
    pub input: Input,
    pub output: Output,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input: Input::Standard,
            output: Output::CharStandard,
        }
    }
}

#[derive(Clone)]
pub enum Input {
    Null,
    Standard,
    Vec(Rc<RefCell<VecDeque<i32>>>),
}

#[derive(Clone)]
pub enum Output {
    Null,
    CharStandard,
    IntStandard,
    Vec(Rc<RefCell<VecDeque<i32>>>),
}
