use std::time::Duration;
use std::io::{Error, ErrorKind};
use reqwest::{Client, header};

pub enum RequestMethod {
    Get, Post, Put, Patch, Delete
}

pub struct MyClient {
    url: String,
    client: Client,
}

impl MyClient {
    pub fn new(url: String) -> Self
    {
        MyClient {
            url,
            client: Client::new()
        }
    }

    pub async fn handle(&mut self, url: &str,
                        method: RequestMethod,
                        content: String,
                        headers: header::HeaderMap,
                        timeout: Duration) -> Result<(String, u128), (Error, u128)>
    {
        let t0 = std::time::Instant::now();
        let url = self.url.clone() + url;
        let req = match method {
            RequestMethod::Get => self.client.get(url),
            RequestMethod::Post => self.client.post(url),
            RequestMethod::Put => self.client.put(url),
            RequestMethod::Patch => self.client.patch(url),
            RequestMethod::Delete => self.client.delete(url),
        };
        match req.timeout(timeout)
            .headers(headers)
            .body(content)
            .send().await {
            Ok(res) => {
                Ok((res.text().await.unwrap(), t0.elapsed().as_millis()))
            }
            Err(_e) => {
                Err((Error::new(ErrorKind::Other, _e.to_string()), t0.elapsed().as_millis()))
            }
        }
    }
}