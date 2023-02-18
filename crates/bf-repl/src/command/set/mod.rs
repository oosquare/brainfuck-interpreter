use common::{Memory, MemoryError};
use snafu::prelude::*;

type Result<T> = std::result::Result<T, SetError>;

#[derive(Snafu, Debug, PartialEq, Eq)]
pub enum SetError {
    #[snafu(display("set cell out of bound"))]
    OutOfBound { source: MemoryError },
    #[snafu(display("value is overflowed"))]
    Overflow { source: MemoryError },
}

pub fn execute(memory: &mut Memory, addr: isize, val: i32) -> Result<()> {
    match memory.set_at(addr, val) {
        Ok(()) => Ok(()),
        Err(e) => match e {
            e @ MemoryError::AccessOutOfBounds { .. } => Err(SetError::OutOfBound { source: e }),
            e @ MemoryError::SetOverflow { .. } => Err(SetError::Overflow { source: e }),
            _ => unreachable!(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set() {
        let mut memory: Memory = Default::default();
        assert_eq!(execute(&mut memory, 0, 1), Ok(()));
        assert_eq!(memory.get_at(0), Ok(1));
        
        assert_eq!(execute(&mut memory, 1, 2), Ok(()));
        assert_eq!(memory.get_at(1), Ok(2));
    }

    #[test]
    fn set_out_of_bound() {
        let mut memory: Memory = Default::default();
        assert!(matches!(execute(&mut memory, -1, 0), Err(SetError::OutOfBound { source: _ })));
    }

    #[test]
    fn set_overflow() {
        let mut memory: Memory = Default::default();
        assert!(matches!(execute(&mut memory, 1, 100000), Err(SetError::Overflow { source: _ })));
    }
}