pub mod php;

use std::{io, sync::Arc, env};
use tokio::sync::Mutex;
use php::*;
use async_trait::async_trait;
use crate::client::MyClient;

pub struct Doings {
    handler: Option<Vec<Box<dyn TestFire>>>,
    // times: usize,
    // duration: Duration,
    client: Arc<Mutex<MyClient>>,
}

#[async_trait]
pub trait TestFire
{
    async fn fire(&mut self, client: &mut MyClient) -> Result<(String, u128), (io::Error, u128)>;
}

impl Doings {
    pub fn new(client: Arc<Mutex<MyClient>>) -> Self {
        Doings {
            handler: None,
            // times,
            // duration,
            client
        }
    }

    pub async fn run(&mut self) -> Result<(), io::Error> {
        if let Some(handlers) = self.handler.take() {
            let mut inx = 1;
            for mut handle in handlers {
                let client = self.client.clone();
                let mut c = client.lock().await;
                match handle.fire(&mut c).await {
                    Ok(res) => println!("{}: ok ---> {}, costs: {}ms", inx, res.0, res.1),
                    Err(e) => println!("{}: fault ---> {}, costs: {}ms", inx, e.0.to_string(), e.1)
                }
                inx += 1;
            }
        }
        Ok(())
    }

    pub fn php_all(&mut self) -> &mut Self
    {
        let test_buff = env::var("BUFF").expect("Need set BUFF!");
        let test_portal = env::var("PORTAL").expect("Need set PORTAL!");
        if test_buff.eq("1") {
            let get_guild = Box::new(buff::GetGuild{
                guild_id: "12387478".to_string(), user_id: "78797089".to_string()
            });
            let get_members = Box::new(buff::GetMembers {
                guild_id: "12387478".to_string(), user_id: "78797089".to_string()
            });
            let list_guild_emoji = Box::new(buff::ListGuildEmoji {
                guild_id: "12387478".to_string()
            });
            let circle_detail = Box::new(buff::CircleDetail {
                post_id: "12387478".to_string(), _type: "all".to_string(), from: None
            });
            let list_dm = Box::new(buff::ListDM {
                user_id: "12387478".to_string(), last_time: 1661165327
            });
            let user_auth = Box::new(buff::UserAuthentication {
                user_id: "12387478".to_string()
            });
            let get_list = Box::new(buff::GetList {
                channel_id: "3242786686".to_string(),
                size: None,
                message_id: "987384656".to_string(),
                behavior: None
            });
            self.php_insert(get_guild);
            self.php_insert(get_members);
            self.php_insert(list_guild_emoji);
            self.php_insert(get_list);
            self.php_insert(circle_detail);
            self.php_insert(list_dm);
            self.php_insert(user_auth);
        }
        if test_portal.eq("1") {
            let get_me = Box::new(portal::GetMe {
                user_ids: "324284728472".to_string(),
                guild_id: None,
                response_type: None
            });
            let get_guilds = Box::new(portal::GetGuilds {
                hash: "324284728472".to_string(),
            });
            self.php_insert(get_guilds);
            self.php_insert(get_me);
        }
        self
    }

    fn php_insert(&mut self, handle: Box<dyn TestFire>)
    {
        if let Some(mut vec) = self.handler.take() {
            vec.push(handle);
            self.handler = Some(vec);
        } else {
            self.handler = Some(vec![handle])
        }
    }
}