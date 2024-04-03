use network::tcp;
use store::Store;
mod command;
mod data_type;
mod decoder;
mod encoder;
mod errors;
mod network;
mod store;

fn main() -> anyhow::Result<()> {
    let store = Store::new();

    let mut tcp_server = tcp::TCPServer::new("127.0.0.1:1523", store.clone())?;
    tcp_server.run()?;

    Ok(())
}
