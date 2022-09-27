use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

    println!("Running on port 3000");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        println!("Connection established");

        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
