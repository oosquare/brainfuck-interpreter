use common::{Memory, AddrRange};
use snafu::prelude::*;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, ViewError>;
pub struct MemoryView<'a> {
    memory: &'a Memory,
    range: AddrRange,
}

impl Display for MemoryView<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let AddrRange { left, right } = self.range;
        
        // First line
        write!(f, "| {:^5} |", "index")?;

        for index in left..=right {
            write!(f, " {index:^5} |")?;
        }

        writeln!(f)?;

        // Second line
        write!(f, "| {:^5} |", "value")?;

        for index in left..=right {
            // Already checked the index.
            write!(f, " {:^5} |", self.memory.get_at(index).unwrap())?;
        }

        writeln!(f)?;

        // Third line
        write!(f, "| {:^5} |", "char")?;

        for index in left..=right {
            // Already checked the index.
            let ch = self.memory.get_at(index).unwrap() as u32;
            let ch = char::from_u32(ch).unwrap_or('�');
            let ch = if ch.is_control() { ' ' } else { ch };
            write!(f, " {ch:^5} |")?;
        }
        Ok(())
    }
}

#[derive(Snafu, Debug, PartialEq, Eq)]
pub enum ViewError {
    #[snafu(display("index in given [{}, {}] is out of [{}, {}]",
    given_range.left, given_range.right, memory_range.left, memory_range.right))]
    OutOfRange {
        given_range: AddrRange,
        memory_range: AddrRange,
    },
    #[snafu(display("[{}, {}] is an invalid range", range.left, range.right))]
    InvalidRange { range: AddrRange },
}

fn get(memory: &Memory, range: AddrRange) -> Result<MemoryView> {
    ensure!(range.left <= range.right, InvalidRangeSnafu { range });

    let AddrRange { left, right } = memory.range();
    ensure!(
        left <= range.left && range.right <= right,
        OutOfRangeSnafu {
            given_range: range,
            memory_range: memory.range()
        }
    );

    Ok(MemoryView { memory, range })
}

pub fn execute(memory: &Memory, range: AddrRange) -> Result<()> {
    let view = get(memory, range)?;
    println!("{view}");
    Ok(())
}

mod tests {
    use super::*;

    #[test]
    fn display() {
        let mut memory: Memory = Default::default();
        memory.add(48).unwrap();
        memory.seek(1).unwrap();
        memory.add(2).unwrap();
        let range = AddrRange { left: 0, right: 4 };
        const OUTPUT: &str = 
"| index |   0   |   1   |   2   |   3   |   4   |
| value |  48   |   2   |   0   |   0   |   0   |
| char  |   0   |       |       |       |       |";
        assert_eq!(format!("{}", get(&memory, range).unwrap()), OUTPUT);
    }
}