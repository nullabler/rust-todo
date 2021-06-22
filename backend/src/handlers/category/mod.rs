use futures::executor::block_on;
use hyper::{Method, Response, StatusCode};
use route_recognizer::Params;
use serde::{Serialize, Deserialize};

use crate::handlers::{get_response_by_status_code, parse_body, ResultResponseHyper, RequestHyper};


pub struct CategoryHandle {
    // list: Vec<Category>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Req {
    pub name: String
}


// struct Category {
    // label: String
// }

// struct Red {
//     fl: String
// }

pub fn configure() -> CategoryHandle {
    CategoryHandle {
        // list: vec![]
    }
}

impl CategoryHandle {
    pub fn call(&self, req: RequestHyper, params: &Params) -> ResultResponseHyper {
        match (req.method(), params.find("action")) {
            (&Method::GET, None) => {
                println!("Req: {:?}", req);
                Ok(Response::new("category index".into()))
            },
            (&Method::POST, Some("create")) => {
                let request: Req = serde_json::from_str(block_on(parse_body(req)).as_str()).unwrap();
                Ok(Response::new(
                    serde_json::to_string(&request).unwrap().into()
                ))
            },
            (&_, _) => get_response_by_status_code(StatusCode::NOT_FOUND)
        }
    }

}
