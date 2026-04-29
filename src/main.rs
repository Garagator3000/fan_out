mod config;
mod sharder;
mod sink;
mod transport;

// Common.
use std::error;
use clap::Parser;
use log::{debug, info};
use std::sync::mpsc::Receiver;

// Own.
use crate::sharder::shard::{HashSharder, Sharder};
use crate::sink::Sink;
use crate::sink::file;
use crate::transport::Transport;

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "config.yaml")]
    config: String,

    #[arg(short, long)]
    log_level: Option<log::LevelFilter>,
}

fn init_logger(cfg: &config::Config, args: &Args) {
    let level = args
    .log_level
    .unwrap_or(cfg.log_level);

    env_logger::Builder::new().filter_level(level).init();
}

fn main() -> Result<(), Box<dyn error::Error>>{
    let args = Args::parse();
    let cfg = config::load(&args.config)?;
    
    init_logger(&cfg, &args);

    debug!("{:?}", cfg);

    let (tx, rx) = std::sync::mpsc::channel();
    let transport = transport::udp::create_udp_transport(cfg.app.port);
    let sharder = HashSharder::new_hash_sharder(cfg.app.shards);
    let sink = file::create_file_sink(&cfg.app.file_pattern, &cfg.app.file_path, cfg.app.shards)?;

    std::thread::spawn(move || {
        worker(rx, sink, sharder)
    });

    transport.run(tx)?;

    info!("service started");

    Ok(())
}

pub fn worker(rx: Receiver<Vec<u8>>, mut sink: sink::file::FileSink, sharder: HashSharder) {
    for msg in rx {
        let shard = sharder.shard(&msg);
        let _ = sink.write(shard, &msg);
        let _ = sink.flush();
    }
}

