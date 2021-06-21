use hyper::{Body, Request, Response, StatusCode};
use route_recognizer::Router;

type ResultResponse = Result<Response<Body>, hyper::Error>;

mod category;

pub async fn routes(req: Request<Body>) -> ResultResponse {
    let mut router = Router::new();
    router.add("/category/*action", category::configure());
    router.add("/category", category::configure());

    match router.recognize(req.uri().path()) {
        Ok(handle) => {
            handle.handler().call(req, handle.params())
        },
        _ => get_response_by_status_code(StatusCode::NOT_FOUND)
    }
}

fn get_response_by_status_code(status_code: StatusCode) -> ResultResponse {
    let mut response = Response::default();
    *response.status_mut() = status_code;
    Ok(response)
}
