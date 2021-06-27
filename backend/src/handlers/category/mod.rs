use futures::executor::block_on;
use hyper::{Method, Response, StatusCode};
use route_recognizer::Params;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

use crate::{Config, handlers::{get_response_by_status_code, parse_body, ResultResponseHyper, RequestHyper}};

pub struct CategoryHandle {
    config: Arc<Mutex<Config>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Category {
    id: i32,
    name: String
}

pub fn configure(config: Arc<Mutex<Config>>) -> CategoryHandle {
    CategoryHandle {
        config
    }
}

impl CategoryHandle {
    pub fn call(&self, req: RequestHyper, params: &Params) -> ResultResponseHyper {
        match (req.method(), params.find("action")) {
            (&Method::GET, None) => {
                let mut conf = self.config.lock().unwrap();
                conf.v.push(9);
                println!("root: {:?}", conf);
                println!("Req: {:?}", req);
                Ok(Response::new("Ok".into()
                    // serde_json::to_string(&self.list).unwrap().into()
                ))
            },

            (&Method::POST, Some("create")) => {
                let _request: Request = serde_json::from_str(block_on(parse_body(req)).as_str()).unwrap();

                let mut conf = self.config.lock().unwrap();
                conf.v.push(7);
                println!("create: {:?}", conf);
                // self.push_category(request.name);
                Ok(Response::new("Ok".into()))
            },
            (&_, _) => get_response_by_status_code(StatusCode::NOT_FOUND)
        }
    }

    // fn push_category(&mut self, name: String) {
    //     let mut id: i32 = 1;
    //     for (_key, val) in self.cache.iter() {
    //         if id < *val {
    //             id = *val;
    //         }
    //     }

    //     &self.cache.entry(name).or_insert(id);
    // }
}
