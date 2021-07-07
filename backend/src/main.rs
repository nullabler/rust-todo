// use mysql_async::prelude::*;

mod app;
mod handlers;
mod route;

use app::App;
use hyper::{Server, Body, Request, Response, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::sync::{
    Arc,
    Mutex,
};



type ResultResponseHyper = Result<Response<Body>, hyper::Error>;
type RequestHyper = Request<Body>;
type RequestApp = Arc<Mutex<App>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // let database_url = "mysql://root:root@mysql:3306/todo";
    // let pool = mysql_async::Pool::new(database_url);
    // let mut conn = pool.get_conn().await?;

    // conn.query_drop(
    //     r"CREATE TABLE payment (
    //         customer_id int not null,
    //         amount int not null,
    //         account_name text
    //     )"
    // ).await?;

    let app = Arc::new(Mutex::new(App::new()));
    let addr = app.lock().unwrap().config.addr().parse().unwrap();

    let make_service = make_service_fn(move |_| {
        let app = Arc::clone(&app);
        async move {
            Ok::<_, hyper::Error>(service_fn(move |_req: RequestHyper| {
                let app = Arc::clone(&app);
                route::configure(app, _req)
            }))
        }
    });


    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}

fn get_response_by_status_code(status_code: StatusCode) -> ResultResponseHyper {
    let mut response = Response::default();
    *response.status_mut() = status_code;
    Ok(response)
}

async fn parse_body(req: RequestHyper) -> String {
    String::from_utf8(
        hyper::body::to_bytes(req).await.unwrap().to_vec()
    ).unwrap()
}
