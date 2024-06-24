mod container;

use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use commons::c2s_data_packet::{C2SDataPacket, Packet};
use commons::rsa_encryption::{RsaEncryption, RsaEncryptionImpl};
use commons::s2c_data_packet::S2CDataPacket;
use commons::serialization::{Serialization, SerializationImpl};
use crate::container::{Container, ContainerImpl};

//TODO create multiple tasks for client
// e.g. one would handle server response, one would handle user input, etc.
fn main() {

    // Serialize the c2s data packet
    let serialized = SerializationImpl.serialize(&C2SDataPacket::new("hello")).unwrap();

    // Create a new tcp stream, connecting to the server
    let mut stream = match TcpStream::connect("127.0.0.1:8080") {
        // if nothing failed return the stream
        Ok(stream) => stream,
        // if the stream failed to connect print error and return
        Err(e) => {
            println!("Failed to connect: {}", e);
            return;
        }
    };

    // if the stream failed to write to server print error and return
    if let Err(e) = stream.write_all(serialized.as_bytes()) {
        println!("Failed to write to server: {}", e);
        return;
    }

    // Shutdown the write part of the stream
    if let Err(e) = stream.shutdown(Shutdown::Write) {
        println!("Failed to shutdown tcp stream: {}", e);
        return;
    }

    // Create a new string to store the response
    let mut response = String::new();

    // If the stream failed to read from server print error and return
    if let Err(e) = stream.read_to_string(&mut response) {
        //                                           ^^^^^^^^ Assign the response string to the server response if nothing failed
        println!("Failed to read from server: {}", e);
        return;
    }

    let s2c: S2CDataPacket = SerializationImpl.deserialize(&response).unwrap();
    //       ^^^ define the type of the packet we are receiving, in this case S2CDataPacket.
    //           I will implement packet signatures and a packet factory detecting what packet it is
    //           based on the signature.

    // Create a new Container storing the private key (s2c is the private key packet)
    let container = ContainerImpl::new(s2c.get_message());



    // Encrypt the message "hello" with the public key from the container
    let encrypted = RsaEncryptionImpl.encrypt(&container.get_key(), &"hello");

    println!("encrypted: {}", encrypted);

    // ^^ Testing encryption
}
