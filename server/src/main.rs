use std::net::TcpListener;
const LOCAL: &str = "127.0.0.1:7878";
fn main() {
    let listener = TcpListener::bind(LOCAL)
    .expect("Could not bind to LOCAL");
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("Connection established");
    }   
}