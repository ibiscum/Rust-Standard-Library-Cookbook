extern crate hyper;
extern crate futures;

use futures::Future;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::service::service_fn;
use std::net::SocketAddr;

fn main() {
    let addr = "[::1]:3000".parse().expect("Failed to parse address");
    run_echo_server(&addr);
}

fn run_echo_server(addr: &SocketAddr) {
    let new_service = || {
        service_fn(|req: Request<Body>| {
            // An easy way to implement routing is
            // to simply match the request's path.
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => handle_root(),
                (&Method::POST, "/echo") => handle_echo(req),
                _ => handle_not_found(),
            }
        })
    };

    let server = Server::bind(addr)
        .serve(new_service)
        .map_err(|err| eprintln!("server error: {}", err));

    hyper::rt::run(server);
}

type ResponseResult = Result<Response<Body>, hyper::Error>;
fn handle_root() -> ResponseResult {
    const MSG: &str = "Try doing a POST at /echo";
    Ok(
        Response::builder()
            .header(CONTENT_TYPE, "text/plain; charset=utf-8")
            .header(CONTENT_LENGTH, MSG.len().to_string())
            .body(Body::from(MSG))
            .expect("Failed to build root response"),
    )
}

fn handle_echo(req: Request<Body>) -> ResponseResult {
    // The echoing is implemented by setting the response's
    // body to the request's body
    Ok(Response::new(req.into_body()))
}

fn handle_not_found() -> ResponseResult {
    // Return a 404 for every unsupported route
    Ok(
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .expect("Failed to build 404 response"),
    )
}
