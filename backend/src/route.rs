use crate::{
    get_response_by_status_code,
    handlers::{CategoryHandler, Handler},
    RequestApp, RequestHyper, ResultResponseHyper,
};
use hyper::StatusCode;
use route_recognizer::Router;
use std::sync::Arc;

pub async fn configure(app: RequestApp, req: RequestHyper) -> ResultResponseHyper {
    let mut router = Router::new();
    // router.add("/category", CategoryHandler::new(Arc::clone(&app)));

    router.add("/category", "category");
    router.add("/category/", "category");
    router.add("/category/:action", "category");

    println!("path: {:?}", req.uri().path());
    match router.recognize(req.uri().path()) {
        Ok(handle) => {
            // handle.handler().call(req, handle.params())
            match *handle.handler() {
                &"category" => {
                    let q = CategoryHandler::new(Arc::clone(&app));
                    q.call(req, handle.params())
                }
                &&_ => get_response_by_status_code(StatusCode::NOT_FOUND),
            }
        }
        _ => get_response_by_status_code(StatusCode::NOT_FOUND),
    }
}
