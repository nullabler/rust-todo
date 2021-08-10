use futures::executor::block_on;
use hyper::{Method, Response, StatusCode};
use route_recognizer::Params;
use serde::{Serialize, Deserialize};
use crate::{RequestApp, get_response_by_status_code, parse_body, ResultResponseHyper, RequestHyper};
use super::Handler;


pub struct CategoryHandler {
    app: RequestApp
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Categoryq2e {
    id: i32,
    name: String
}


impl Handler for CategoryHandler {
    fn new (app: RequestApp) -> CategoryHandler {
        CategoryHandler {
            app
        }
    }

    fn call (&self, req: RequestHyper, params: &Params) -> ResultResponseHyper {
        println!("params: {:?}", &params.find("action"));
        match (req.method(), params.find("action")) {
            (&Method::GET, None) => {
                let mut app = self.app.lock().unwrap();
                // let resp = config.cache.entry("/category".to_string()).or_insert("{\"Ok\"}".to_string());
                app.cache.v.push(9);
                println!("cache root: {:?}", app.cache);
                println!("Req: {:?}", req);
                println!("db_url: {:?}", app.config.db_url());
                let category_list = app.repository.category.query(&app.db);
                dbg!(category_list);
                // app.repository.task.query(&app.db);
                Ok(Response::new(
                    "Ok".into()
                    // serde_json::to_string(&category_list).unwrap().into()
                ))
            },

            (&Method::POST, Some("create")) => {
                let _request: Request = serde_json::from_str(block_on(parse_body(req)).as_str()).unwrap();

                let mut app = self.app.lock().unwrap();
                app.cache.v.push(7);
                println!("cache create: {:?}", app.cache);
                // self.push_category(request.name);
                Ok(Response::new("Ok".into()))
            },
            (&_, _) => get_response_by_status_code(StatusCode::NOT_FOUND)
        }
    }
}
