use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;
fn main() {
    let mut _stream = TcpStream::connect("localhost:3000").unwrap();
    _stream.write("Hello".as_bytes()).unwrap();

    let mut buffer = [0;5];
    _stream.read(&mut buffer).unwrap();
    println!("Response form server : {:?}", str::from_utf8(&buffer).unwrap());
    
}
