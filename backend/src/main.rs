use hyper::{Server, Body, Request};
use hyper::service::{make_service_fn, service_fn};
use std::sync::{
    Arc,
    Mutex,
};

mod handlers;

#[derive(Debug)]
pub struct Config {
    pub v: Vec<i32>
}

impl Config {
    fn new() -> Self {
        Config {
            v: vec![]
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([0, 0, 0, 0], 7878).into();
    let config = Arc::new(Mutex::new(Config::new()));

    let make_service = make_service_fn(move |_| {
        let config = Arc::clone(&config);
        async move {
            Ok::<_, hyper::Error>(service_fn(move |_req: Request<Body>| {
                let config = Arc::clone(&config);
                handlers::routes(config, _req)
            }))
        }
    });


    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    // server.await?;

    // Ok(())
    // println!("{:?}", server);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}

// async fn test(v: Arc<Mutex<Vec<i32>>>, _req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
//     let mut v = v.lock().unwrap();
//     v.push(5);
//     println!("{:?}", v);
//     Ok(
//         Response::new(
//             Body::from(format!("Request: red"))
//         )
//     )
// }
