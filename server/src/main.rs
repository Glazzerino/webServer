use std::net::{TcpListener,TcpStream};
use std::io::prelude::*;
use std::fs;
const LOCAL: &str = "127.0.0.1:7878";
fn main() {
    let listener = TcpListener::bind(LOCAL)
    .expect("Could not bind to LOCAL");
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(&stream);
    }   
}

fn handle_connection(mut stream: &TcpStream) {
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}",
    String::from_utf8_lossy(&buffer[..]));
    
    let contents = fs::read_to_string("hello.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}",contents);
    stream.write(response.as_bytes()).unwrap();
}