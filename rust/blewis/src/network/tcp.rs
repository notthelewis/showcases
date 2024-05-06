use std::net::SocketAddr;

use super::tcp_cnx::TcpCnx;
use crate::{command::decode_command, store::Store};
use anyhow::Context;
use tokio::{io::AsyncReadExt, net::TcpListener};

pub async fn run_tcp(port: &str, store: Store) -> anyhow::Result<()> {
    let listener = TcpListener::bind(port).await?;

    loop {
        let (cnx, sockaddr) = listener.accept().await.context("couldn't accept client")?;
        let s = TcpCnx::new(cnx);
        let store = store.clone();

        tokio::spawn(async move {
            handle_connection(store, s, sockaddr).await;
        });
    }
}

async fn handle_connection(store: Store, mut s: TcpCnx, sockaddr: SocketAddr) {
    // TODO: Handshake
    
    let bytes_read = s.cnx.read(&mut s.buf).await;
    if bytes_read.is_err() {
        eprintln!(
            "err: {} when reading from socket: {}",
            bytes_read.unwrap_err(),
            sockaddr
        );
        return;
    }

    let command = decode_command(&mut s.buf);
    if command.is_err() {
        // TODO: Handle errors better
        eprintln!(
            "err: {} when decoding command from socket: {}",
            command.unwrap_err(),
            sockaddr
        );
        return;
    }

    let result = command.unwrap().execute(store);
    println!("{result:?}");
}
