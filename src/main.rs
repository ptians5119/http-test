use std::sync::Arc;
use tokio::sync::Mutex;
use std::thread;

mod factory;
mod client;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut threads = Vec::with_capacity(4);
    for i in 0..4 {
        let client = Arc::new(Mutex::new(client::MyClient::new()));
        threads.insert(i, tokio::spawn(async move {
            println!("thread {} is start..", i);
            factory::Doings::new(
                client.clone(),
                i as i32
            ).init().await.run().await
        }));
    }

    for th in threads {
        let _ = th.await;
    }
    // let client = Arc::new(Mutex::new(client::MyClient::new()));
    // let _ = factory::Doings::new(
    //     client.clone(),
    //     1
    // ).init().await.run().await.unwrap();
    Ok(())
}
