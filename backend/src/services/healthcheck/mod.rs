use hyper::{Body, Method, Request, Response, StatusCode, service::Service};
use hyper::service::{make_service_fn, service_fn};
// use hyper::service::make::MakeServiceFn;

async fn routes(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/healthcheck") => get_response_by_status_code(StatusCode::OK),
        _ => get_response_by_status_code(StatusCode::NOT_FOUND)
    }
}

fn get_response_by_status_code(status_code: StatusCode) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::default();
    *response.status_mut() = status_code;
    Ok(response)
}

pub fn configure() -> Service {
    make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(routes))
    })
}
