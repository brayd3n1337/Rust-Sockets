use std::io::{Read, Write};
use std::net::{TcpStream, Shutdown};

fn main() {
    let mut stream = match TcpStream::connect("127.0.0.1:8080") {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            return;
        }
    };

    if let Err(e) = stream.write_all(b"Hello World!\n") {
        eprintln!("Failed to write to server: {}", e);
        return;
    }

    // Shutdown the write part of the stream
    if let Err(e) = stream.shutdown(Shutdown::Write) {
        eprintln!("Failed to shutdown tcp stream: {}", e);
        return;
    }

    let mut response = String::new();
    if let Err(e) = stream.read_to_string(&mut response) {
        eprintln!("Failed to read from server: {}", e);
        return;
    }

    println!("Server response: {}", response);
}