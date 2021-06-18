use hyper::{Body, Response};

pub fn call() -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("test echo")))
}
