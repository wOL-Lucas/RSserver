use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

mod routes;
use routes::Router;

fn main() {
    let router = Router::new();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let cloned_router = router.clone(); // Clone the router for the thread

        println!("Connection established!");

        thread::spawn(move || {
            handle_client(cloned_router, stream);
        });
    }
}

fn handle_client(router: Router, mut stream: TcpStream) {
    let mut request = String::new();
    let mut reader = BufReader::new(&mut stream);

    if reader.read_to_string(&mut request).is_err() {
        return;
    }

    let http_header = request.lines().next().unwrap_or("").trim();

    if router.get_routes().contains_key(http_header) {
        router.send_response(stream, http_header);
    } else {
        router.send_error(stream);
    }
}
