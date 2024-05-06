mod command;
mod data_type;
mod decoder;
mod encoder;
mod errors;
mod network;
mod store;
use network::tcp::run_tcp;
use store::Store;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let store = Store::new();

    run_tcp("127.0.0.1:1523", store).await?;

    Ok(())
}
