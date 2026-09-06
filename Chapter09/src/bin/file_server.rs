extern crate futures;
extern crate hyper;

use futures::Future;
use futures::sync::oneshot;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::service::service_fn;
use std::net::SocketAddr;
use std::thread;
use std::fs::File;
use std::io::{self, copy};

fn main() {
    let addr = "[::1]:3000".parse().expect("Failed to parse address");
    run_file_server(&addr);
}

fn run_file_server(addr: &SocketAddr) {
    let new_service = || {
        service_fn(|req: Request<Body>| {
            // Setting up our routes
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => handle_root(),
                (&Method::GET, path) => handle_get_file(path),
                _ => handle_invalid_method(),
            }
        })
    };

    let server = Server::bind(addr)
        .serve(new_service)
        .map_err(|err| eprintln!("server error: {}", err));

    hyper::rt::run(server);
}

// Because we don't want the entire server to block when serving a file,
// we are going to return a response wrapped in a future
type ResponseFuture = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;
fn handle_root() -> ResponseFuture {
    // Send the landing page
    send_file_or_404("index.html")
}

fn handle_get_file(file: &str) -> ResponseFuture {
    // Send whatever page was requested or fall back to a 404 page
    send_file_or_404(file)
}

fn handle_invalid_method() -> ResponseFuture {
    // Send a page telling the user that the method he used is not supported
    let response_future = send_file_or_404("invalid_method.html")
        // Set the correct status code
        .and_then(|response| {
            let (mut parts, body) = response.into_parts();
            parts.status = StatusCode::METHOD_NOT_ALLOWED;
            Ok(Response::from_parts(parts, body))
        });
    Box::new(response_future)
}

// Send a future containing a response with the requested file or a 404 page
fn send_file_or_404(path: &str) -> ResponseFuture {
    // Sanitize the input to prevent unwanted data access
    let path = sanitize_path(path);

    let response_future = try_to_send_file(&path)
        .then(|result| match result {
            Ok(Ok(response)) => Ok(response),
            _ => send_404_response(),
        });
    Box::new(response_future)
}

// Return a requested file in a future of Result<Response, io::Error>
// to indicate whether it exists or not
type ResponseResultFuture = Box<dyn Future<Item = Result<Response<Body>, io::Error>, Error = oneshot::Canceled> + Send>;
fn try_to_send_file(file: &str) -> ResponseResultFuture {
    // Prepend "files/" to the file
    let path = path_on_disk(file);
    // Load the file in a separate thread into memory.
    // As soon as it's done, send it back through a channel
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => {
                println!("Failed to find file: {}", path);
                // Send error through channel
                tx.send(Err(err)).expect("Send error on file not found");
                return;
            }
        };

        // buf is our in-memory representation of the file
        let mut buf: Vec<u8> = Vec::new();
        match copy(&mut file, &mut buf) {
            Ok(_) => {
                println!("Sending file: {}", path);
                // Detect the content type by checking the file extension
                // or fall back to plaintext
                let content_type = get_content_type(&path).unwrap_or("text/plain; charset=utf-8");
                let res = Response::builder()
                    .header(CONTENT_LENGTH, buf.len().to_string())
                    .header(CONTENT_TYPE, content_type)
                    .body(Body::from(buf))
                    .expect("Failed to build file response");
                // Send file through channel
                tx.send(Ok(res))
                    .expect("Send error on successful file read");
            }
            Err(err) => {
                // Send error through channel
                tx.send(Err(err)).expect("Send error on error reading file");
            }
        };
    });
    Box::new(rx)
}

fn send_404_response() -> Result<Response<Body>, hyper::Error> {
    // If the 404 page doesn't exist, send fallback text instead
    const ERROR_MSG: &str = "Failed to find \"File not found\" page. How ironic\n";
    Ok(
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(CONTENT_LENGTH, ERROR_MSG.len().to_string())
            .header(CONTENT_TYPE, "text/plain; charset=utf-8")
            .body(Body::from(ERROR_MSG))
            .expect("Failed to build fallback 404 response"),
    )
}

fn sanitize_path(path: &str) -> String {
    // Normalize the separators for the next steps
    path.replace("\\", "/")
        // Prevent the user from going up the filesystem
        .replace("../", "")
        // If the path comes straigh from the router,
        // it will begin with a slash
        .trim_start_matches(|c| c == '/')
        // Remove slashes at the end as we only serve files
        .trim_end_matches(|c| c == '/')
        .to_string()
}

fn path_on_disk(path_to_file: &str) -> String {
    "files/".to_string() + path_to_file
}

fn get_content_type(file: &str) -> Option<&'static str> {
    // Check the file extension and return the respective MIME type
    let pos = file.rfind('.')? + 1;
    let mime_type = match &file[pos..] {
        "txt" => "text/plain; charset=utf-8",
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        // This list can be extended for all types your server should support
        _ => return None,
    };
    Some(mime_type)
}
