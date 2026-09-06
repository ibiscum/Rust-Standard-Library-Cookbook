extern crate futures;
extern crate hyper;

use futures::Future;
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server};
use std::net::SocketAddr;

const MESSAGE: &str = "Hello World!";

fn main() {
    // [::1] is the loopback address for IPv6, 3000 is a port
    let addr = "[::1]:3000".parse().expect("Failed to parse address");
    run_with_service_function(&addr);
}

fn run_with_service_function(addr: &SocketAddr) {
    // Hyper is based on Services, which are construct that
    // handle how to respond to requests.
    // service_fn is a convenience function
    // that build a service out of a closure
    let new_service = || {
        service_fn(|_: Request<Body>| {
        println!("Got a connection!");
        // Return a Response with a body of type hyper::Body
            Ok::<Response<Body>, hyper::Error>(Response::builder()
                // Add header specifying content type as plain text
                .header(CONTENT_TYPE, "text/plain; charset=utf-8")
                // Add header specifying the length of the message in bytes
                .header(CONTENT_LENGTH, MESSAGE.len().to_string())
                // Add body with our message
                .body(Body::from(MESSAGE))
                .expect("Failed to build hello world response"))
        })
    };

    let server = Server::bind(addr)
        .serve(new_service)
        .map_err(|err| eprintln!("server error: {}", err));

    hyper::rt::run(server);
}

// The following function does the same, but uses an explicitely created
// struct HelloWorld that implements the Service trait
#[allow(dead_code)]
fn run_with_service_struct(addr: &SocketAddr) {
    run_with_service_function(addr);
}

#[allow(dead_code)]
type _UnusedFutureAlias = Box<dyn Future<Item = Response<Body>, Error = hyper::Error>>;
