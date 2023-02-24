use super::DEFAULT_LEN;

#[derive(Clone)]
pub struct Config {
    pub len: usize,
    pub addr: Addr,
    pub cell: Cell,
    pub overflow: Overflow,
    pub eof: Eof,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            len: DEFAULT_LEN,
            addr: Addr::Unsigned,
            cell: Cell::I8,
            overflow: Overflow::Error,
            eof: Eof::Ignore,
        }
    }
}

#[derive(Clone)]
pub enum Addr {
    Unsigned,
    Signed,
}

#[derive(Clone)]
pub enum Cell {
    I8,
    I32,
}

#[derive(Clone)]
pub enum Overflow {
    Error,
    Wrap,
}

#[derive(Clone)]
pub enum Eof {
    Zero,
    Keep,
    Ignore,
}
