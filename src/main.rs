use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::env;

async fn handle(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match req.uri().path() {
        "/healthz" => Ok(Response::new(Body::from("ok"))),
        _ => Ok(Response::new(Body::from("hello from ploy"))),
    }
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".into());
    let addr = format!("0.0.0.0:{}", port).parse().unwrap();
    let make_svc = make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(handle)) });
    let server = Server::bind(&addr).serve(make_svc);
    println!("listening on {}", addr);
    if let Err(e) = server.await { eprintln!("server error: {}", e); }
}
