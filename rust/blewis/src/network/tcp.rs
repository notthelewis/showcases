use super::tcp_cnx::TcpCnx;
use crate::{command::decode_command, store::Store};
use anyhow::{Context, Ok};
use std::{io::Read, net::TcpListener, thread};

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
    pub fn run(&mut self) -> anyhow::Result<()> {
        println!("starting TCP Server");
        loop {
            for stream in self.listener.incoming() {
                let store = self.store.clone();

                // TODO: Beter thread management
                thread::spawn(move || {
                    let cnx = stream.unwrap();
                    let mut s = TcpCnx::new(cnx);

                    if let Err(e) = s.cnx.read(&mut s.buf) {
                        println!("error: {:?} while reading from client", e);
                        return;
                    }

                    let cmd = decode_command(&mut s.buf);
                    if cmd.is_err() {
                        println!("{}", cmd.unwrap_err());
                        return;
                    }
                    let cmd = cmd.unwrap();
                    println!("valid command parsed");

                    let result = cmd.execute(store);
                    if result.is_none() {
                        println!("no result");
                        return;
                    }

                    let result = result.unwrap();
                    println!("valid result: {result}");
                });
            }
        }
    }
}

// impl TCPServer {
//     pub fn run(&mut self) -> anyhow::Result<()> {
//         // TODO: Async IO
//         // TODO: Store connections in the TCPServer struct, for graceful shutdown etc
//
//         loop {
//             let (cnx, _) = self.listener.accept().context("couldn't get client")?;
//             let mut s = TcpCnx::new(cnx);
//
//             // TODO: Handshake
//
//             let bytes_read = s.cnx.read(&mut s.buf)?;
//             println!("{bytes_read}");
//             let command = decode_command(&mut s.buf)?;
//
//             let result = command.execute(self.store.clone());
//             println!("{result:?}");
//         }
//     }
// }
