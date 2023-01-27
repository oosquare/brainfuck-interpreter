#![allow(unused)]

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Config {
    pub input: Input,
    pub output: Output
}

#[derive(Clone)]
pub enum Input {
    Null,
    Standard,
    Vec(Rc<RefCell<VecDeque<i8>>>),
}

#[derive(Clone)]
pub enum Output {
    CharStandard,
    IntStandard,
    Vec(Rc<RefCell<VecDeque<i32>>>),
}