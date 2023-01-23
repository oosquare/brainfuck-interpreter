#![allow(unused)]

pub trait InputStream {
    fn read(&mut self) -> u8;
}

pub trait OutputStream {
    fn write(&mut self, content: u8);
}