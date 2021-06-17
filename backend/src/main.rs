use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};

// async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
//     Ok(Response::new(Body::from("Hello World")))
// }

// async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
//     Ok(Response::new("Hello world 2".into()))
// }

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from("This root"))),
        (&Method::GET, "/echo") => Ok(Response::new(req.into_body())),
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([0, 0, 0, 0], 7878).into();

    let service = make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(echo))
    });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
