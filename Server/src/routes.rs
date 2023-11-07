use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    sync::Arc,
    collections::HashMap,
    env,
};
extern crate jsonwebtoken;
use jsonwebtoken::{encode, Header, EncodingKey, Algorithm, Validation};

pub struct Claims{
    subject: String,
    issued: i64,
    expires: i64,
}


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

    pub fn send_response(&self, mut stream: TcpStream, request_header: &str) {
        let status_line = "HTTP/1.1 200 OK";
        let binding = "".to_string();

        let file_name = self.routes.get(request_header).unwrap_or(&binding);
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
        let request_header = BufReader::new(&mut stream).lines().next().unwrap().unwrap();
        println!("Request: {}", request_header);

        if self.routes.contains_key(&request_header) {
            self.send_response(stream, &request_header);
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

            self.clone().handle(stream);
        }
    }

    pub fn generate_token() -> String{
        let claim = Claims {
            subject: "testing".to_string(),
            issued: 0,
            expires: 0,
        };

        let header = Header::new(Algorithm::HS256);
        return encode(&header, &claim, &EncodingKey::from_secret(Self::get_secret_key().as_ref())).unwrap();
    }

    pub fn get_key() -> String {
        let key_file = fs::read_to_string("../.key").unwrap();
        return key_file
    }

    pub fn get_secret_key() -> String {
        let secret_key = env::var("SECRET_KEY").unwrap_or_else(|_| "test".to_string());
        return secret_key
    }

}
