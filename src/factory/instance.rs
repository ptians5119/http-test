use std::{fs, io::{Result, Read, ErrorKind, Error}};
use crate::client::RequestMethod as Method;
use serde_json;
use serde_json::Value;

struct Instance {
    name: String,
    url: String,
    times: usize,
    duration: u64,
    modules: Vec<Module>
}

struct Module {
    name: String,
    reqs: Vec<Request>
}

struct Request {
    name: String,
    api: String,
    headers: Vec<(String, String)>,
    method: Method,
    body: String,
    stores: Vec<String>,
}

impl Instance {
    pub(crate) fn init_instance_of_file() -> Result<Vec<Self>>
    {
        let content = Instance::read_file()?;
        if let Ok(root) = serde_json::from_str::<Value>(content.as_str()) {
            let instances = root.get_instances()?;
            Ok(instances)
        } else {
            Err(Error::new(ErrorKind::Other, "[init]decode from file get some wrongs!"))
        }
    }

    fn read_file() -> Result<String>
    {
        let mut file = fs::File::open("request.json")?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    /// 默认执行次数为1，循环间隔为0
    pub(crate) fn new() -> Self
    {
        Self {
            name: "".to_string(),
            url: "".to_string(),
            times: 1,
            duration: 0,
            modules: vec![]
        }
    }

    pub(crate) fn name(mut self, val: String) -> Instance
    {
        self.name = val; self
    }

    pub(crate) fn url(mut self, val: String) -> Instance
    {
        self.url = val; self
    }

    pub(crate) fn times(mut self, val: usize) -> Instance
    {
        self.times = val; self
    }

    pub(crate) fn duration(mut self, val: u64) -> Instance
    {
        self.duration = val; self
    }

    pub(crate) fn modules(mut self, val: Vec<Module>) -> Instance
    {
        self.modules = val; self
    }

    pub(crate) fn ok(self) -> bool
    {
        self.name.len()!=0 && self.url.len()!=0 && !self.modules.is_empty()
    }
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

    pub(crate) fn ok(self) -> bool
    {
        self.name.len()!=0 && !self.reqs.is_empty()
    }
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
                vec.push((obj.0.to_owned(), obj.1.to_string()))
            }
        }
        self.headers = vec;
        self
    }

    pub(crate) fn method(mut self, val: String) -> Request
    {
        self.method = match val.as_str() {
            "get" => Method::Get,
            "post" => Method::Post,
            "put" => Method::Put,
            "patch" => Method::Patch,
            "delete" => Method::Delete,
            _ => Method::None
        };
        self
    }

    pub(crate) fn body(mut self, val: String) -> Request
    {
        self.body = val;
        self
    }

    pub(crate) fn stores(mut self, val: Vec<String>) -> Request
    {
        self.stores = val;
        self
    }

    pub(crate) fn ok(self) -> bool
    {
        self.name.len()!=0 && self.api.len()!=0 && self.method!=Method::None
    }
}

trait JsonUtils {
    fn get_instances(self) -> Result<Vec<Instance>>;
    fn get_modules(self) -> Result<Vec<Module>>;
    fn get_request(self) -> Result<Vec<Request>>;
}

impl JsonUtils for Value {
    fn get_instances(self) -> Result<Vec<Instance>> {
        let mut instances = vec![];
        if let Some(objects) = self.as_object() {
            for obj in objects {
                if let Some(sub_objects) = obj.1.as_object() {
                    let mut instance = Instance::new();
                    let mut modules = Value::Null;
                    for sub_obj in sub_objects {
                        match sub_obj.0.as_str()? {
                            "enable" => {
                                if let Some(1) = sub_obj.1.as_u64() {
                                    continue
                                } else {
                                    break
                                }
                            }
                            "base" => {
                                instance.name(obj.0.to_owned())
                                    .url(sub_obj.1["url"].to_string())
                                    .times(sub_obj.1["times"].as_u64()? as usize)
                                    .duration(sub_obj.1["duration"].as_u64()?)
                            }
                            _ => {
                                modules[sub_obj.0] = sub_obj.1.to_owned();
                            }
                        }
                    }
                    instance.modules(modules.get_modules()?);
                    if instance.ok() {
                        instances.push(instance);
                    }
                } else {
                    return Err(Error::new(ErrorKind::Other, "[ins]sub isn't a object!"))
                }
            }
            Ok(vec![])
        } else {
            Err(Error::new(ErrorKind::Other, "[ins]origin isn't a object!"))
        }
    }

    fn get_modules(self) -> Result<Vec<Module>> {
        if let Some(objects) = self.as_object() {
            let mut modules = vec![];
            for obj in objects {
                let module_name = obj.0.to_owned();
                match obj.0.as_str() {
                    "enable" => {
                        if let Some(1) = sub_obj.1.as_u64() {
                            continue
                        } else {
                            break
                        }
                    }
                    "reqs" => {
                        modules.push(Module::new()
                            .name(module_name)
                            .reqs(obj.1.get_request()?)
                            .into())
                    }
                    _ => continue
                }
            }
            Ok(modules)
        } else {
            Err(Error::new(ErrorKind::Other, "[mod]origin isn't a object!"))
        }
    }

    fn get_request(self) -> Result<Vec<Request>> {
        if let Some(vec) = self.as_array() {
            let mut requests = vec![];
            for req in vec {
                let request = Request::new()
                    .name(req["name"].to_string())
                    .api(req["api"].to_string())
                    .headers(req["headers"].clone())
                    .method(req["method"].to_string())
                    .body(req["body"].to_string())
                    .stores(vec![]);
                if request.ok() {
                    requests.push(request.into())
                }
            }
            Ok(requests)
        } else {
            Err(Error::new(ErrorKind::Other, "[req]origin isn't a array!"))
        }
    }
}