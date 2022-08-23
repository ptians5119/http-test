use reqwest::header::{HeaderMap, HeaderValue, HeaderName};

pub mod buff {
    use serde_with::skip_serializing_none;
    use async_trait::async_trait;
    use std::borrow::BorrowMut;
    use crate::client::{MyClient, RequestMethod::*};
    use crate::factory::TestFire;
    use std::io;
    use std::time::Duration;
    use reqwest::header::HeaderMap;
    use serde::Serialize;
    use crate::factory::php::header;

    #[derive(Serialize, Clone)]
    pub struct GetGuild {
        pub(crate) guild_id: String,
        pub(crate) user_id: String,
    }

    #[async_trait]
    impl TestFire for GetGuild {
        async fn fire(&mut self, client: &mut MyClient) -> Result<(String,u128), (io::Error,u128)> {
            let body = serde_json::to_string(&self).unwrap();
            let mut map = HeaderMap::new();
            let duration = Duration::from_millis(500);
            header(map.borrow_mut(),
                   "Content-Type".to_string(),
                   "application/x-www-form-urlencoded".to_string());
            client.handle("/api/guild/getGuild",
                                Post,
                                body,
                                map,
                                duration).await
        }
    }

    #[derive(Serialize, Clone)]
    pub struct GetMembers {
        pub(crate) guild_id: String,
        pub(crate) user_id: String,
    }

    #[async_trait]
    impl TestFire for GetMembers {
        async fn fire(&mut self, client: &mut MyClient) -> Result<(String,u128), (io::Error,u128)> {
            let body = serde_json::to_string(&self).unwrap();
            let mut map = HeaderMap::new();
            let duration = Duration::from_millis(500);
            header(map.borrow_mut(),
                   "Content-Type".to_string(),
                   "application/json".to_string());
            client.handle("/api/guild/getMember",
                          Post,
                          body,
                          map,
                          duration).await
        }
    }

    #[derive(Serialize, Clone)]
    pub struct ListGuildEmoji {
        pub(crate) guild_id: String,
    }

    #[async_trait]
    impl TestFire for ListGuildEmoji {
        async fn fire(&mut self, client: &mut MyClient) -> Result<(String,u128), (io::Error,u128)> {
            let body = serde_json::to_string(&self).unwrap();
            let mut map = HeaderMap::new();
            let duration = Duration::from_millis(500);
            header(map.borrow_mut(),
                   "Content-Type".to_string(),
                   "application/json".to_string());
            client.handle("/api/emojis/lists",
                          Post,
                          body,
                          map,
                          duration).await
        }
    }

    #[derive(Serialize, Clone)]
    pub struct ListDM {
        pub(crate) user_id: String,
        pub(crate) last_time: i64,
    }

    #[async_trait]
    impl TestFire for ListDM {
        async fn fire(&mut self, client: &mut MyClient) -> Result<(String,u128), (io::Error,u128)> {
            let body = serde_json::to_string(&self).unwrap();
            let mut map = HeaderMap::new();
            let duration = Duration::from_millis(500);
            header(map.borrow_mut(),
                   "Content-Type".to_string(),
                   "application/json".to_string());
            client.handle("/api/dm/dmList",
                          Post,
                          body,
                          map,
                          duration).await
        }
    }

    #[derive(Serialize, Clone)]
    pub struct UserAuthentication {
        pub(crate) user_id: String,
    }

    #[async_trait]
    impl TestFire for UserAuthentication {
        async fn fire(&mut self, client: &mut MyClient) -> Result<(String,u128), (io::Error,u128)> {
            let body = serde_json::to_string(&self).unwrap();
            let mut map = HeaderMap::new();
            let duration = Duration::from_millis(500);
            header(map.borrow_mut(),
                   "Content-Type".to_string(),
                   "application/x-www-form-urlencoded".to_string());
            client.handle("/api/ID/CheckByUid",
                          Post,
                          body,
                          map,
                          duration).await
        }
    }

    #[skip_serializing_none]
    #[derive(Serialize, Clone)]
    pub struct CircleDetail {
        pub(crate) post_id: String,
        #[serde(rename(serialize = "type"))]
        pub(crate) _type: String,
        pub(crate) from: Option<String>,
    }

    #[async_trait]
    impl TestFire for CircleDetail {
        async fn fire(&mut self, client: &mut MyClient) -> Result<(String,u128), (io::Error,u128)> {
            let body = serde_json::to_string(&self).unwrap();
            let mut map = HeaderMap::new();
            let duration = Duration::from_millis(500);
            header(map.borrow_mut(),
                   "Content-Type".to_string(),
                   "application/json".to_string());
            client.handle("/api/circlePost/detail",
                          Post,
                          body,
                          map,
                          duration).await
        }
    }

    #[skip_serializing_none]
    #[derive(Serialize, Clone)]
    pub struct GetList {
        pub(crate) channel_id: String,
        pub(crate) size: Option<String>,
        pub(crate) message_id: String,
        pub(crate) behavior: Option<String>,
    }

    #[async_trait]
    impl TestFire for GetList {
        async fn fire(&mut self, client: &mut MyClient) -> Result<(String,u128), (io::Error,u128)> {
            let body = serde_json::to_string(&self).unwrap();
            let mut map = HeaderMap::new();
            let duration = Duration::from_millis(500);
            header(map.borrow_mut(),
                   "Content-Type".to_string(),
                   "application/x-www-form-urlencoded".to_string());
            client.handle("/api/message/getList",
                          Post,
                          body,
                          map,
                          duration).await
        }
    }
}

pub mod portal {
    use serde_with::skip_serializing_none;
    use async_trait::async_trait;
    use std::borrow::BorrowMut;
    use crate::client::{MyClient, RequestMethod::*};
    use crate::factory::TestFire;
    use std::io;
    use std::time::Duration;
    use reqwest::header::HeaderMap;
    use serde::Serialize;
    use crate::factory::php::header;

    #[skip_serializing_none]
    #[derive(Serialize, Clone)]
    pub struct GetMe {
        pub(crate) user_ids: String,
        pub(crate) guild_id: Option<String>,
        pub(crate) response_type: Option<String>,
    }

    #[async_trait]
    impl TestFire for GetMe {
        async fn fire(&mut self, client: &mut MyClient) -> Result<(String,u128), (io::Error,u128)> {
            let body = serde_json::to_string(&self).unwrap();
            let mut map = HeaderMap::new();
            let duration = Duration::from_millis(500);
            header(map.borrow_mut(),
                   "Content-Type".to_string(),
                   "application/x-www-form-urlencoded".to_string());
            header(map.borrow_mut(),
                   "Authorization".to_string(),
                   "adfafjhakfaoiu9988d76ahk".to_string());
            client.handle("/api/user/getUser",
                          Post,
                          body,
                          map,
                          duration).await
        }
    }

    #[derive(Serialize, Clone)]
    pub struct GetGuilds {
        pub(crate) hash: String
    }

    #[async_trait]
    impl TestFire for GetGuilds {
        async fn fire(&mut self, client: &mut MyClient) -> Result<(String,u128), (io::Error,u128)> {
            let body = serde_json::to_string(&self).unwrap();
            let mut map = HeaderMap::new();
            let duration = Duration::from_millis(500);
            header(map.borrow_mut(),
                   "Content-Type".to_string(),
                   "application/json".to_string());
            client.handle("/api/guild/myGuild2?user_id=9373432489",
                          Post,
                          body,
                          map,
                          duration).await
        }
    }
}

pub fn header(map: &mut HeaderMap, name: String, value: String)
{
    let name = HeaderName::from_bytes(name.as_bytes()).unwrap();
    let value = HeaderValue::from_bytes(value.as_bytes()).unwrap();
    map.insert(name, value);
}