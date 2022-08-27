pub mod instance;

use std::{io, sync::Arc};
use tokio::sync::Mutex;
use crate::client::MyClient;
use crate::factory::instance::Instance;

pub struct Doings {
    handler: Option<Vec<Instance>>,
    client: Arc<Mutex<MyClient>>,
}

impl Doings {
    pub(crate) fn new(client: Arc<Mutex<MyClient>>) -> Self {
        Doings {
            handler: None,
            client
        }
    }

    pub(crate) async fn init(&mut self) -> &mut Doings {
        self.handler = match Instance::init_instance_of_file() {
            Ok(vec) => Some(vec),
            Err(_e) => {
                println!("init instance error: {}", _e.to_string());
                None
            }
        };
        self
    }

    pub(crate) async fn run(&mut self) -> Result<(), io::Error> {
        if let Some(handlers) = self.handler.take() {
            for handle in handlers {
                let client = self.client.clone();
                let mut c = client.lock().await;
                let _ = handle.fire(&mut c).await?;
            }
        }
        Ok(())
    }
}