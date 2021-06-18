pub mod healthcheck;
// use hyper::{Request, Response, Body, Method, StatusCode};

// mod echo;
// mod healthcheck;


// pub async fn routes(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
//     match (req.method(), req.uri().path()) {
//         (&Method::GET, "/") => Ok(Response::new(Body::from("This root"))),
//         (&Method::GET, "/echo") => echo::call(),
//         (&Method::GET, "/h") => healthcheck::call(),
//         (&Method::GET, "/healthcheck") => get_response_by_status_code(StatusCode::OK),
//         _ => get_response_by_status_code(StatusCode::NOT_FOUND)
//     }
// }

// fn get_response_by_status_code(status_code: StatusCode) -> Result<Response<Body>, hyper::Error> {
//     let mut response = Response::default();
//     *response.status_mut() = status_code;
//     Ok(response)
// }
