use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};
use std::sync::Arc;

mod routes;
use routes::Router;

fn main() {
    let router = Router::new();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let router_clone: &Router = &router;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");

        thread::spawn(move || {
            router_clone.handle(stream);
        });
    }
}
