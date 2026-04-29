use std::sync::mpsc::Sender;
use log::debug;

const MAX_BUFFER_SIZE: usize = 2048; // MTU 1500 + запас

pub struct UDPTransport {
    addr: String,
}

pub fn create_udp_transport(port: u16) -> UDPTransport {
    UDPTransport {
        addr: format!("0.0.0.0:{port}")
    }
}

impl super::Transport for UDPTransport {
    fn run(&self, tx: Sender<Vec<u8>>) -> std::io::Result<()>
    {
        let socket = std::net::UdpSocket::bind(&self.addr)?;
        let mut buf = [0u8; MAX_BUFFER_SIZE];

        loop {
            let (len, _) = socket.recv_from(&mut buf)?;
            let mut data = buf[..len].to_vec();
            data.push(b'\n');

            tx.send(data).unwrap();
            debug!("data sent")
        }
    }
}
