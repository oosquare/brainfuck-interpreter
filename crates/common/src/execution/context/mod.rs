use crate::execution::memory::{
    config::Config as MemoryConfig, Builder as MemoryBuilder, Memory,
};
use crate::execution::stream::{
    config::Config as StreamConfig, Builder as StreamBuilder, InStream, OutStream,
};

pub struct Context {
    pub memory: Memory,
    pub in_stream: Box<dyn InStream>,
    pub out_stream: Box<dyn OutStream>,
}

impl Context {
    pub fn new(memory_config: MemoryConfig, stream_config: StreamConfig) -> Self {
        let MemoryConfig {
            len,
            addr,
            cell,
            overflow,
            eof,
        } = memory_config;

        let memory = MemoryBuilder::new()
            .len(len)
            .addr(addr)
            .cell(cell)
            .overflow(overflow)
            .eof(eof)
            .build();

        let StreamConfig { input, output } = stream_config;
        let (in_stream, out_stream) = StreamBuilder::new().input(input).output(output).build();

        Self {
            memory,
            in_stream,
            out_stream,
        }
    }
}
