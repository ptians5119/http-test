use std::{fs, io::{Result, Read, ErrorKind, Error}, time};
use crate::client::{RequestMethod as Method, MyClient};
use serde_json;
use serde_json::Value;
use chrono;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

#[derive(Debug)]
pub struct Instance {
    name: String,
    url: String,
    times: usize,
    duration: u64,
    timeout: u64,
    modules: Vec<Module>
}

#[derive(Debug)]
struct Module {
    name: String,
    reqs: Vec<Request>
}

#[derive(Debug)]
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

    pub(crate) async fn fire(self, client: &mut MyClient) -> Result<()>
    {
        let t0 = time::Instant::now();
        println!(">>>>>>>>> Test for module: {}, begin at {}",
                 &self.name,
                 chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        );
        let mut mod_inx = 1;
        for module in self.modules {
            println!("{}.{} :", mod_inx, &module.name);
            mod_inx += 1;
            let mut req_inx = 1;
            for req in module.reqs {
                let url = self.url.clone() + &req.api;
                println!("{}). {}", req_inx, &url);
                // 处理headers
                let mut map = HeaderMap::new();
                for (k, v) in req.headers {
                    header(&mut map, k.clone(), v.clone());
                }
                let timeout = time::Duration::from_millis(self.timeout);
                let res = client.handle(url.as_str(),
                              req.method,
                              req.body.to_owned(),
                              map,
                              timeout).await;
                match res {
                    Ok(msg) => println!("Ok,elapsed:{}ms ==> {}", msg.1, &msg.0),
                    Err(e) => println!("Error,elapsed:{}ms ==> {}", e.1, e.0.to_string())
                }
                req_inx += 1;
            }
        }
        println!("<<<<<<<<< End, total elapsed {}ms", t0.elapsed().as_millis());
        Ok(())
    }

    fn read_file() -> Result<String>
    {
        let mut file = fs::File::open("request.json")?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    /// 默认执行次数为1，循环间隔为0, 请求超时时间100ms
    fn new() -> Self
    {
        Self {
            name: "".to_string(),
            url: "".to_string(),
            times: 1,
            duration: 0,
            timeout: 100,
            modules: vec![]
        }
    }

    fn name(mut self, val: String) -> Instance
    {
        self.name = val; self
    }

    fn url(mut self, val: String) -> Instance
    {
        self.url = val; self
    }

    fn times(mut self, val: usize) -> Instance
    {
        self.times = val; self
    }

    fn duration(mut self, val: u64) -> Instance
    {
        self.duration = val; self
    }

    fn timeout(mut self, val: u64) -> Instance
    {
        self.timeout = val; self
    }

    fn modules(mut self, val: Vec<Module>) -> Instance
    {
        self.modules = val; self
    }

    fn ok(&self) -> bool
    {
        self.name.len()!=0 && self.url.len()!=0 && !self.modules.is_empty()
    }
}

impl Module {
    fn new() -> Self
    {
        Self {
            name: "".to_string(),
            reqs: vec![]
        }
    }

    fn name(mut self, val: String) -> Module
    {
        self.name = val; self
    }

    fn reqs(mut self, val: Vec<Request>) -> Module
    {
        self.reqs = val; self
    }

    fn ok(&self) -> bool
    {
        self.name.len()!=0 && !self.reqs.is_empty()
    }
}

impl Request {
    fn new() -> Self
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

    fn name(mut self, val: String) -> Request
    {
        self.name = val; self
    }

    fn api(mut self, val: String) -> Request
    {
        self.api = val; self
    }

    fn headers(mut self, json: Value) -> Request
    {
        let mut vec = vec![];
        if let Some(objects) = json.as_object() {
            for obj in objects {
                vec.push((obj.0.to_owned(), obj.1.to_string()))
            }
        }
        self.headers = vec; self
    }

    fn method(mut self, val: String) -> Request
    {
        self.method = Method::to(val); self
    }

    fn body(mut self, val: String) -> Request
    {
        self.body = val; self
    }

    fn stores(mut self, val: Vec<String>) -> Request
    {
        self.stores = val; self
    }

    fn ok(&self) -> bool
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
        // println!("{:?}", &self);
        if let Some(objects) = self.as_object() {
            let mut instances = vec![];
            for obj in objects {
                if let Some(sub_objects) = obj.1.as_object() {
                    if sub_objects["enable"].as_u64() == Some(0) {
                        continue
                    }
                    let mut tmp = (
                        "".to_string(),
                        "".to_string(),
                        0, 0, 0
                    );
                    let mut modules = Value::Null;
                    for sub_obj in sub_objects {
                        match sub_obj.0.as_str() {
                            "enable" => {
                                continue
                            }
                            "base" => {
                                tmp = (
                                    obj.0.to_owned(),
                                    sub_obj.1["url"].as_str().unwrap().to_string(),
                                    sub_obj.1["times"].as_u64().unwrap() as usize,
                                    sub_obj.1["duration"].as_u64().unwrap(),
                                    sub_obj.1["timeout"].as_u64().unwrap()
                                    );
                            }
                            _ => {
                                modules[sub_obj.0] = sub_obj.1.to_owned();
                            }
                        }
                    }
                    // let aa = modules.get_modules()?;
                    // println!("{:?}", &aa);
                    let instance = Instance::new()
                        .name(tmp.0)
                        .url(tmp.1)
                        .times(tmp.2)
                        .duration(tmp.3)
                        .timeout(tmp.4)
                        .modules(modules.get_modules()?);
                    if instance.ok() {
                        instances.push(instance);
                    }
                } else {
                    return Err(Error::new(ErrorKind::Other, "[ins]sub isn't a object!"))
                }
            }
            Ok(instances)
        } else {
            Err(Error::new(ErrorKind::Other, "[ins]origin isn't a object!"))
        }
    }

    fn get_modules(self) -> Result<Vec<Module>> {
        if let Some(objects) = self.as_object() {
            let mut modules = vec![];
            for obj in objects {
                let module_name = obj.0.to_owned();
                if obj.1["enable"].eq(&1) {
                    let module = Module::new()
                        .name(module_name)
                        .reqs(obj.1["reqs"].clone().get_request()?);
                    if module.ok() {
                        modules.push(module.into())
                    }
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
                    .name(req["name"].as_str().unwrap().to_string())
                    .api(req["api"].as_str().unwrap().to_string())
                    .headers(req["headers"].clone())
                    .method(req["method"].as_str().unwrap().to_string())
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

pub fn header(map: &mut HeaderMap, name: String, value: String)
{
    let name = HeaderName::from_bytes(name.as_bytes()).unwrap();
    let value = HeaderValue::from_bytes(value.as_bytes()).unwrap();
    map.insert(name, value);
}