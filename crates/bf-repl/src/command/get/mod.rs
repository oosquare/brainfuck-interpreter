use common::execution::memory::{Memory, MemoryError};
use snafu::prelude::*;

type Result<T> = std::result::Result<T, GetError>;

#[derive(Snafu, Debug, PartialEq, Eq)]
pub enum GetError {
    #[snafu(display("get cell out of bound"))]
    OutOfBound { source: MemoryError },
}

pub fn execute(memory: &Memory, addr: isize) -> Result<i32> {
    match memory.get_at(addr) {
        Ok(res) => Ok(res),
        Err(e) => Err(GetError::OutOfBound { source: e }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let mut memory: Memory = Default::default();
        memory.set_at(0, 1).unwrap();
        memory.set_at(1, 2).unwrap();
        assert_eq!(execute(&memory, 0), Ok(1));
        assert_eq!(execute(&memory, 1), Ok(2));
    }

    #[test]
    fn get_out_of_bound() {
        let memory: Memory = Default::default();
        assert!(matches!(
            execute(&memory, -1),
            Err(GetError::OutOfBound { .. })
        ));
    }
}
