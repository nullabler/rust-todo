mod app;
mod handlers;

use app::App;
use hyper::{Server, Body, Request};
use hyper::service::{make_service_fn, service_fn};
use std::{collections::HashMap, sync::{
    Arc,
    Mutex,
}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = Arc::new(Mutex::new(App::new()));
    let addr = app.lock().unwrap().config.addr().parse().unwrap();

    let make_service = make_service_fn(move |_| {
        let app = Arc::clone(&app);
        async move {
            Ok::<_, hyper::Error>(service_fn(move |_req: Request<Body>| {
                let app = Arc::clone(&app);
                handlers::routes(app, _req)
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
