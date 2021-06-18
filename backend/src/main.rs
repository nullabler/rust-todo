use hyper::Server;
// use hyper::service::{make_service_fn, service_fn};

mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([0, 0, 0, 0], 7878).into();

    // let service = make_service_fn(|_| async {
    //     Ok::<_, hyper::Error>(service_fn(services::routes))
    // });

    let server = Server::bind(&addr).serve(services::healthcheck::configure());

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
