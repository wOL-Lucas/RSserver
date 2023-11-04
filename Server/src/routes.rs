use std::collections::HashMap;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};
use std::sync::Arc;


pub struct Router {
    routes: HashMap<String, String>,
}

impl Router {
    fn create_routes() -> HashMap<String, String> {
        let mut routes: HashMap<String, String> = HashMap::new();
        routes.insert("GET /home HTTP/1.1".to_string(), "./public/views/home/home.html".to_string());
        routes
    }

    pub fn new() -> Router {
        let routes = Router::create_routes();
        Router { routes }
    }

    pub fn send_response(&self, mut stream: TcpStream, http_header: &str) {
        let status_line = "HTTP/1.1 200 OK";
        let binding = "".to_string();

        let file_name = self.routes.get(http_header).unwrap_or(&binding);
        let contents = fs::read_to_string(file_name).unwrap();
        let contents_length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {contents_length}\r\n\r\n{contents}");
        
        stream.write_all(response.as_bytes()).unwrap();
    }

    pub fn send_error(&self, mut stream: TcpStream) {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let file_name = "./public/views/errors/not_found/404.html";
        let contents = fs::read_to_string(file_name).unwrap();
        let contents_length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {contents_length}\r\n\r\n{contents}");
        
        stream.write_all(response.as_bytes()).unwrap();
    }

    pub fn handle(&self, mut stream: TcpStream) {
        let http_header = BufReader::new(&mut stream).lines().next().unwrap().unwrap();
        println!("Request: {}", http_header);

        if self.routes.contains_key(&http_header) {
            self.send_response(stream, &http_header);
        } else {
            self.send_error(stream);
        }
    }

    pub fn get_routes(&self) -> &HashMap<String, String> {
        &self.routes
    }

    pub fn clone(&self) -> Router {
        Router { routes: self.routes.clone() }
    }
    
    pub fn init_server(&self) {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        for stream in listener.incoming(){
            let stream = stream.unwrap();
            println!("Connection established!");
            self.clone().handle(stream);
        }
    }
}
