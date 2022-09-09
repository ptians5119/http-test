pub mod instance;
pub mod module;
pub mod request;

use std::{sync::Arc, io::{ Result, Error, ErrorKind}};
use tokio::sync::Mutex;
use crate::client::MyClient;
use crate::factory::instance::Instance;
use serde_json::Value;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use module::*;
use request::*;

pub struct Doings {
    handler: Option<Vec<Instance>>,
    client: Arc<Mutex<MyClient>>,
    th_no: i32
}

impl Doings {
    pub(crate) fn new(client: Arc<Mutex<MyClient>>, no: i32) -> Self {
        Doings {
            handler: None,
            client,
            th_no: no
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

    pub(crate) async fn run(&mut self) -> Result<()> {
        if let Some(handlers) = self.handler.take() {
            for handle in handlers {
                let client = self.client.clone();
                let mut c = client.lock().await;
                let _ = handle.fire(&mut c, self.th_no).await?;
            }
        }
        Ok(())
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

                    let instance = Instance::new()
                        .name(tmp.0)
                        .url(tmp.1)
                        .times(tmp.2)
                        .duration(tmp.3)
                        .timeout(tmp.4)
                        .modules(modules.get_modules()?)
                        .ok()?;
                    instances.push(instance);
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
                        .reqs(obj.1["reqs"].clone().get_request()?)
                        .ok()?;
                    modules.push(module.into())
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
                    .stores(vec![])
                    .ok()?;
                requests.push(request.into());
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