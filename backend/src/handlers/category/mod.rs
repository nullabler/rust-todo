use hyper::{Body, Response};

pub fn index() -> Result<Response<Body>, hyper::Error>{
    Ok(Response::new("category index".into()))
}
