use std::{fs, io::{Result, Read, ErrorKind, Error}, time};
use crate::client::MyClient;
use chrono;
use reqwest::header::HeaderMap;
use super::{header, JsonUtils, Module, Value};

#[derive(Debug)]
pub struct Instance {
    name: String,
    url: String,
    times: usize,
    duration: u64,
    timeout: u64,
    modules: Vec<Module>
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

    pub(crate) async fn fire(self, client: &mut MyClient, th: i32) -> Result<()>
    {
        let mut times = 0;
        let t0 = time::Instant::now();
        let mut message = vec![];
        message.push(format!(">>>>>>>>> Test for thread: {}, module: {}, begin at {}",
                             th,
                             &self.name,
                             chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
        let mut req_inx = 1;
        loop {
            let mut output = "".to_string();
            if times > self.times {
                break
            } else {
                times += 1;
            }
            let mut mod_inx = 1;
            for module in &self.modules {
                let str = format!("{}.{} :\n", mod_inx, &module.name);
                output += str.as_str();
                mod_inx += 1;
                for req in &module.reqs {
                    let url = self.url.clone() + &req.api;
                    let str = format!("{}). {}\n [headers] ==> {}\n [body] ==> {}\n", req_inx, &url, Instance::header_output(&req.headers), &req.body);
                    output += str.as_str();
                    // 处理headers
                    let mut map = HeaderMap::new();
                    for (k, v) in &req.headers {
                        header(&mut map, k.clone(), v.clone());
                    }
                    let timeout = time::Duration::from_millis(self.timeout);
                    let res = client.handle(url.as_str(),
                                            &req.method,
                                            req.body.to_owned(),
                                            map,
                                            timeout).await;
                    let str = match res {
                        Ok(msg) => format!(" [send] ==> ok \n [elapsed] ==> {}ms\n [msg] ==> {}", msg.1, &msg.0),
                        Err(e) => format!(" [send] ==> err \n [elapsed] ==> {}ms\n [msg] ==> {}", e.1, e.0.to_string())
                    };
                    output += str.as_str();
                    req_inx += 1;
                }
            }
            message.push(output);
        }
        for m in message {
            println!("{}", m);
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

    fn header_output(headers: &Vec<(String, String)>) -> Value {
        let mut json = Value::Null;
        for (k, v) in headers {
            json[k] = v.to_owned().into();
        }
        json
    }

    /// 默认执行次数为1，循环间隔为0, 请求超时时间100ms
    pub(crate) fn new() -> Self
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

    pub(crate) fn timeout(mut self, val: u64) -> Instance
    {
        self.timeout = val; self
    }

    pub(crate) fn modules(mut self, val: Vec<Module>) -> Instance
    {
        self.modules = val; self
    }

    pub(crate) fn ok(self) -> Result<Instance>
    {
        if self.name.len()!=0 && self.url.len()!=0 && !self.modules.is_empty() {
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::Other, "new instance invalid"))
        }
    }
}