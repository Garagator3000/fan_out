pub trait Sink {
    fn write(&mut self, shard: usize, data: &[u8]) -> std::io::Result<()>;
}

pub mod file;
