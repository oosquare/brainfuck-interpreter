use bf_repl::Repl;

fn main() {
    let memory_config = Default::default();
    let stream_config = Default::default();
    let mut repl = Repl::new(memory_config, stream_config);
    repl.run();
}
