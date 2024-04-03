use std::{io::Read, net::TcpListener};

use anyhow::{Context, Ok};

use crate::{command_interpreter::decode_command, store::Store};

use super::tcp_cnx::TcpCnx;

pub(crate) struct TCPServer {
    listener: TcpListener,
    store: Store,
}

impl TCPServer {
    pub fn new(port: &str, store: Store) -> anyhow::Result<Self> {
        Ok(TCPServer {
            listener: TcpListener::bind(port)
                .with_context(|| format!("Should bind to port {port}"))?,
            store,
        })
    }
}

impl TCPServer {
    fn run(&mut self) -> anyhow::Result<()> {
        // TODO: Async IO
        // TODO: Store connections in the TCPServer struct, for graceful shutdown etc

        loop {
            let (cnx, _) = self.listener.accept().context("couldn't get client")?;
            let mut s = TcpCnx::new(cnx);

            // TODO: Handshake

            let bytes_read = s.cnx.read(&mut s.buf)?;
            println!("{bytes_read}");
            let command = decode_command(&mut s.buf)?;

            let result = command.execute(self.store.clone());
            println!("{result:?}")
        }
    }
}
