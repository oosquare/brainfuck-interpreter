use super::{MemoryError, Result};
use crate::interpreter::stream::EOF;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct AddrRange {
    pub left: isize,
    pub right: isize,
}

impl AddrRange {
    pub fn len(&self) -> usize {
        (self.right - self.left + 1) as usize
    }
}

pub trait AddrStrategy {
    /// Return the initial value the pointer should contain.
    fn initial(&self) -> isize {
        0
    }

    /// Calculate `addr + offset`. Return `None` when `addr + offset` is out of bounds.
    fn seek(&self, addr: isize, offset: isize) -> Result<isize>;

    /// Calculate the actual address.
    fn calc(&self, addr: isize) -> usize;

    /// Get the abstract address range.
    fn range(&self) -> AddrRange;
}

pub struct UnsignedAddrStrategy {
    len: usize,
}

impl UnsignedAddrStrategy {
    pub fn new(len: usize) -> Self {
        Self { len }
    }
}

impl AddrStrategy for UnsignedAddrStrategy {
    fn seek(&self, addr: isize, offset: isize) -> Result<isize> {
        let target = addr + offset;

        if 0 <= target && target < self.len as isize {
            Ok(target)
        } else {
            Err(MemoryError::OutOfBounds {
                now_position: addr,
                offset,
                range: self.range(),
            })
        }
    }

    fn calc(&self, addr: isize) -> usize {
        addr as usize
    }

    fn range(&self) -> AddrRange {
        AddrRange {
            left: 0,
            right: self.len as isize - 1,
        }
    }
}

pub struct SignedAddrStrategy {
    half_len: usize,
}

impl SignedAddrStrategy {
    pub fn new(half_len: usize) -> Self {
        Self { half_len }
    }
}

impl AddrStrategy for SignedAddrStrategy {
    fn seek(&self, addr: isize, offset: isize) -> Result<isize> {
        let target = addr + offset;

        if -(self.half_len as isize) <= target && target < self.half_len as isize {
            Ok(target)
        } else {
            Err(MemoryError::OutOfBounds {
                now_position: addr,
                offset,
                range: self.range(),
            })
        }
    }

    fn calc(&self, addr: isize) -> usize {
        addr as usize + self.half_len
    }

    fn range(&self) -> AddrRange {
        AddrRange {
            left: -(self.half_len as isize),
            right: self.half_len as isize - 1,
        }
    }
}

pub trait CellStrategy {
    fn is_overflowed(&self, num: i64) -> bool;

    fn wrap(&self, num: i64) -> i32;
}

pub struct I8CellStrategy {}

impl CellStrategy for I8CellStrategy {
    fn is_overflowed(&self, num: i64) -> bool {
        num < i8::MIN as i64 || num > i8::MAX as i64
    }

    fn wrap(&self, num: i64) -> i32 {
        num as i8 as i32
    }
}

pub struct I32CellStrategy {}

impl CellStrategy for I32CellStrategy {
    fn is_overflowed(&self, num: i64) -> bool {
        num < i32::MIN as i64 || num > i32::MAX as i64
    }

    fn wrap(&self, num: i64) -> i32 {
        num as i32
    }
}

pub trait OverflowStrategy {
    /// Calculate and check the value for the `add` operation.
    fn add(&self, cell_strategy: &dyn CellStrategy, before: i32, add: i32) -> Result<i32>;
}

pub struct ErrorOverflowStrategy {}

impl OverflowStrategy for ErrorOverflowStrategy {
    fn add(&self, cell_strategy: &dyn CellStrategy, before: i32, add: i32) -> Result<i32> {
        let res = before as i64 + add as i64;

        if cell_strategy.is_overflowed(res) {
            Err(MemoryError::Overflow { before, add })
        } else {
            Ok(res as i32)
        }
    }
}

pub struct WrapOverflowStrategy {}

impl OverflowStrategy for WrapOverflowStrategy {
    fn add(&self, cell_strategy: &dyn CellStrategy, before: i32, add: i32) -> Result<i32> {
        let res = before as i64 + add as i64;

        if cell_strategy.is_overflowed(res) {
            Ok(cell_strategy.wrap(res))
        } else {
            Ok(res as i32)
        }
    }
}

pub trait EofStrategy {
    fn check(&self, input: i8) -> Option<i8>;
}

#[derive(Debug)]
pub struct ZeroEofStrategy {}

/// Turn EOF to 0.
impl EofStrategy for ZeroEofStrategy {
    fn check(&self, input: i8) -> Option<i8> {
        if input == EOF {
            Some(0)
        } else {
            Some(input)
        }
    }
}

/// Keep EOF.
pub struct KeepEofStrategy {}

impl EofStrategy for KeepEofStrategy {
    fn check(&self, input: i8) -> Option<i8> {
        Some(input)
    }
}

/// Ignore this input if it's EOF.
pub struct IgnoreEofStrategy {}

impl EofStrategy for IgnoreEofStrategy {
    fn check(&self, input: i8) -> Option<i8> {
        if input == EOF {
            None
        } else {
            Some(input)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned_addr_strategy() {
        let r = UnsignedAddrStrategy::new(5);
        assert_eq!(r.seek(0, 2), Ok(2));
        assert_eq!(
            r.seek(0, 5),
            Err(MemoryError::OutOfBounds {
                now_position: 0,
                offset: 5,
                range: AddrRange { left: 0, right: 4 }
            })
        );
        assert_eq!(r.calc(4), 4);
    }

    #[test]
    fn signed_address_strategy() {
        let r = SignedAddrStrategy::new(5);
        assert_eq!(r.seek(0, -5), Ok(-5));
        assert_eq!(
            r.seek(0, -6),
            Err(MemoryError::OutOfBounds {
                now_position: 0,
                offset: -6,
                range: AddrRange { left: -5, right: 4 }
            })
        );
        assert_eq!(r.calc(4), 9);
    }

    #[test]
    fn i8_cell_strategy() {
        let c = I8CellStrategy {};
        assert!(!c.is_overflowed(127));
        assert!(c.is_overflowed(128));
        assert!(!c.is_overflowed(-128));
        assert!(c.is_overflowed(-129));

        assert_eq!(c.wrap(127), 127);
        assert_eq!(c.wrap(128), -128);
        assert_eq!(c.wrap(-129), 127);
        assert_eq!(c.wrap(1121), 97);
        assert_eq!(c.wrap(-1211), 69);
        assert_eq!(c.wrap(-1111), -87);
    }

    #[test]
    fn i32_cell_strategy() {
        let c = I32CellStrategy {};
        // i32::MAX = 2147483647, i32::MIN = -2147483648
        assert!(c.is_overflowed(2147483648i64));
        assert!(!c.is_overflowed(-2147483648i64));
        assert!(c.is_overflowed(-2147483649i64));

        assert_eq!(c.wrap(-2147483649i64), 2147483647);
        assert_eq!(c.wrap(-2147483648i64 - 2147483647i64 - 1i64), 0);
    }

    #[test]
    fn error_overflow_strategy() {
        let o = ErrorOverflowStrategy {};
        let c = I8CellStrategy {};
        assert_eq!(o.add(&c, 0, 1), Ok(1));
        assert_eq!(
            o.add(&c, 127, 1),
            Err(MemoryError::Overflow {
                before: 127,
                add: 1
            })
        );
    }

    #[test]
    fn wrap_overflow_strategy() {
        let o = WrapOverflowStrategy {};
        let c = I8CellStrategy {};
        assert_eq!(o.add(&c, 0, 1), Ok(1));
        assert_eq!(o.add(&c, 127, 1), Ok(-128));
    }
}
