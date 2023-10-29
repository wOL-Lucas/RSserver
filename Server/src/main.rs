use std::{
    fs,
    io::{prelude::*,BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connections(stream);
        });
    }   

}

fn handle_connections(mut stream: TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}",String::from_utf8_lossy(&buffer[..]));

    let get_request = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get_request){
        send_response(stream);
    }else{
        send_error(stream);
    }

}
 
fn send_response(mut stream:TcpStream){
    let (status_line, response_content) = ("HTTP/1.1 200 OK", "<h1>Hello World</h1>");
    let content_length = response_content.len();
    let response = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{response_content}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn send_error(mut stream:TcpStream){
    let (status_line, response_content) = ("HTTP/1.1 404 NOT FOUND", "<h1>Hmm, where is it?</h1>");
    let content_length = response_content.len();
    let response = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{response_content}");

    stream.write_all(response.as_bytes()).unwrap();
}