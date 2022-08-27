use crate::client::RequestMethod as Method;
use super::{Value, Result, Error, ErrorKind};

#[derive(Debug)]
pub struct Request {
    name: String,
    pub(crate) api: String,
    pub(crate) headers: Vec<(String, String)>,
    pub(crate) method: Method,
    pub(crate) body: String,
    stores: Vec<String>,
}

impl Request {
    pub(crate) fn new() -> Self
    {
        Self {
            name: "".to_string(),
            api: "".to_string(),
            headers: vec![],
            method: Method::None,
            body: "".to_string(),
            stores: vec![]
        }
    }

    pub(crate) fn name(mut self, val: String) -> Request
    {
        self.name = val; self
    }

    pub(crate) fn api(mut self, val: String) -> Request
    {
        self.api = val; self
    }

    pub(crate) fn headers(mut self, json: Value) -> Request
    {
        let mut vec = vec![];
        if let Some(objects) = json.as_object() {
            for obj in objects {
                vec.push((obj.0.to_owned(), obj.1.as_str().unwrap().to_string()))
            }
        }
        self.headers = vec; self
    }

    pub(crate) fn method(mut self, val: String) -> Request
    {
        self.method = Method::to(val); self
    }

    pub(crate) fn body(mut self, val: String) -> Request
    {
        self.body = val; self
    }

    pub(crate) fn stores(mut self, val: Vec<String>) -> Request
    {
        self.stores = val; self
    }

    pub(crate) fn ok(self) -> Result<Request>
    {
        if self.name.len()!=0 && self.api.len()!=0 && self.method!=Method::None {
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::Other, "new request invalid"))
        }
    }
}