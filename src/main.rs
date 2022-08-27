use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

mod factory;
mod client;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let client = Arc::new(Mutex::new(client::MyClient::new()));
    let _ = factory::Doings::new(
        client.clone()
    ).init().await.run().await.unwrap();

    Ok(())
}
