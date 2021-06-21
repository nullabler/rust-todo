use futures::StreamExt;
use hyper::{Body, Method, Request, Response, StatusCode};
use route_recognizer::Params;

use std::{collections::HashMap, convert::TryInto};
use url::form_urlencoded;
// use serde::{Serialize, Deserialize};

use crate::handlers::{get_response_by_status_code, ResultResponse};

pub struct CategoryHandle {
    // list: Vec<Category>
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
    pub fn call(&self, req: Request<Body>, params: &Params) -> ResultResponse {
        match (req.method(), params.find("action")) {
            (&Method::GET, None) => {
                println!("Req: {:?}", req);
                Ok(Response::new("category index".into()))
            },
            (&Method::POST, Some("create")) => {
                // let b = hyper::body::to_bytes(req).await?;
                // let res = form_urlencoded::parse(b.as_ref())
                    // .into_owned().collect::<HashMap<String, String>>();
                let b = hyper::body::to_bytes(&req.body());

                let p = form_urlencoded::parse(b.into())
                .into_owned()
                .collect::<HashMap<String, String>>();

                println!("res: {:?}", p);

                Ok(Response::new("category create".into()))
            },
            (&_, _) => get_response_by_status_code(StatusCode::NOT_FOUND)
        }
    }

}
