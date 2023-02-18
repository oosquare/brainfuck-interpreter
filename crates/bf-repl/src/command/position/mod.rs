use common::execution::memory::Memory;

pub fn execute(memory: &Memory) -> isize {
    memory.position()
}
