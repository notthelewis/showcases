use std::net::{TcpListener, TcpStream};

use anyhow::Context;

use super::interface::NetworkLayer;

pub(crate) struct TCPServer {
    listener: TcpListener,
}

impl TCPServer {
    fn new(port: &str) -> anyhow::Result<Self> {
        let listener =
            TcpListener::bind(port).with_context(|| format!("Should bind to port: {port}"))?;

        Ok(TCPServer { listener })
    }

}

impl NetworkLayer<TcpStream, bytes::Bytes, bytes::Bytes> for TCPServer {
    fn send_message(cnx: TcpStream, to_send: bytes::Bytes) -> anyhow::Result<()> {
        todo!()
    }

    fn recv_message(cnx: TcpStream, recv_buf: bytes::Bytes) -> anyhow::Result<usize> {
        todo!()
    }
}
