use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use commons::c2s_data_packet::{C2SDataPacket, Packet};
use commons::serialization::{Serialization, SerializationImpl};

fn main() {
    let c2s_data_packet = C2SDataPacket::new("hello");

    let serializer = SerializationImpl;

    // serialize packet
    let serialized = serializer.serialize(&c2s_data_packet).unwrap();

    let mut stream = match TcpStream::connect("127.0.0.1:8080") {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            return;
        }
    };

    if let Err(e) = stream.write_all(serialized.as_bytes()) {
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

    println!("server response: {}", response);

}
