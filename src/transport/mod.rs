pub mod udp;

pub trait Transport {
    fn run(&self, tx: std::sync::mpsc::Sender<Vec<u8>>) ->std::io::Result<()>;
}
