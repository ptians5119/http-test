use super::Request;
use super::{Result, Error, ErrorKind};

#[derive(Debug)]
pub struct Module {
    pub(crate) name: String,
    pub(crate) reqs: Vec<Request>
}

impl Module {
    pub(crate) fn new() -> Self
    {
        Self {
            name: "".to_string(),
            reqs: vec![]
        }
    }

    pub(crate) fn name(mut self, val: String) -> Module
    {
        self.name = val; self
    }

    pub(crate) fn reqs(mut self, val: Vec<Request>) -> Module
    {
        self.reqs = val; self
    }

    pub(crate) fn ok(self) -> Result<Module>
    {
        if self.name.len()!=0 && !self.reqs.is_empty() {
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::Other, "new module invalid"))
        }
    }
}