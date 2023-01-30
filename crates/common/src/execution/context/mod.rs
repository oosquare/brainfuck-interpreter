use crate::execution::memory::{config::Config as MemoryConfig, Builder as MemoryBuilder, Memory};
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
        let memory = MemoryBuilder::with_config(memory_config).build();
        let (in_stream, out_stream) = StreamBuilder::with_config(stream_config).build();

        Self {
            memory,
            in_stream,
            out_stream,
        }
    }
}
