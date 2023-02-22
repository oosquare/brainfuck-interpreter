use common::execution::memory::{Memory, MemoryError};

use snafu::prelude::*;

type Result<T> = std::result::Result<T, AddError>;

#[derive(Snafu, Debug, PartialEq, Eq)]
pub enum AddError {
    #[snafu(display("add value to cell out of bound"))]
    OutOfBound { source: MemoryError },
    #[snafu(display("result is overflowed"))]
    Overflow { source: MemoryError },
}

pub fn execute(memory: &mut Memory, addr: isize, val: i32) -> Result<()> {
    match memory.add_at(addr, val) {
        Ok(()) => Ok(()),
        Err(e) => match e {
            e @ MemoryError::AccessOutOfBounds { .. } => Err(AddError::OutOfBound { source: e }),
            e @ MemoryError::AddOverflow { .. } => Err(AddError::Overflow { source: e }),
            _ => unreachable!(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let mut memory: Memory = Default::default();
        assert_eq!(execute(&mut memory, 0, 1), Ok(()));
        assert_eq!(memory.get_at(0), Ok(1));

        assert_eq!(execute(&mut memory, 0, 2), Ok(()));
        assert_eq!(memory.get_at(0), Ok(3));
    }

    #[test]
    fn add_out_of_bound() {
        let mut memory: Memory = Default::default();
        assert!(matches!(
            execute(&mut memory, -1, 0),
            Err(AddError::OutOfBound { source: _ })
        ));
    }

    #[test]
    fn add_overflow() {
        let mut memory: Memory = Default::default();
        assert!(matches!(
            execute(&mut memory, 1, 100000),
            Err(AddError::Overflow { source: _ })
        ));
    }
}
