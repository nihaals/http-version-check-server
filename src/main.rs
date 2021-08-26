use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::{env, net::SocketAddr};

async fn handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let http_version = match req.version() {
        hyper::Version::HTTP_09 => "HTTP/0.9",
        hyper::Version::HTTP_10 => "HTTP/1.0",
        hyper::Version::HTTP_11 => "HTTP/1.1",
        hyper::Version::HTTP_2 => "HTTP/2.0",
        hyper::Version::HTTP_3 => "HTTP/3.0",
        _ => "Unknown",
    };

    Ok(Response::new(Body::from(http_version)))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8000".to_string());
    let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(handler)) });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
