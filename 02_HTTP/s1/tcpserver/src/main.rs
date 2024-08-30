use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000 ...");

    for stream in listener.incoming(){
        let mut _stream = stream.unwrap();
        println!("Connection Established!");
        let mut buffer = [0;1024];
        _stream.read(&mut buffer).unwrap();
        _stream.write(&mut buffer).unwrap();
    }
}
