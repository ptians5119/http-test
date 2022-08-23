use std::time::Duration;
use serde::Serialize;
use std::env;
use std::sync::Arc;
use dotenv;
use tokio::sync::Mutex;

mod factory;
mod client;

#[derive(Serialize)]
struct Body {
    channel_id: String,
    message_id: String,
    max: i64
}



#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();

    let php_url = if let Ok(url) = env::var("PHP_URL") {
        Some(url)
    } else {
        None
    };

    let times = env::var("TIMES").expect("Need set TIMES as loop count!");
    let times = times.parse::<usize>().unwrap();

    let duration = env::var("DURATION").expect("Need set DURATION as loop duration (ms)!");
    let duration = duration.parse::<u64>().unwrap();

    if let Some(url) = php_url {
        let client = Arc::new(Mutex::new(client::MyClient::new(url)));
        let mut count = 0;
        loop {
            if count >= times {
                break
            } else {
                count += 1;
                println!(">>>>>> A new loop coming...");
            }
            let _ = factory::Doings::new(
                // 1,
                // Duration::from_micros(50),
                client.clone()
            ).php_all().run().await?;
            std::thread::sleep(Duration::from_micros(duration));
        }
    }

    Ok(())
}
