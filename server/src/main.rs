use std::io::{Read, Write};
use std::net::TcpListener;
use commons::c2s_data_packet::C2SDataPacket;
use commons::packet::Packet;
use commons::rsa_encryption::{RsaEncryption, RsaEncryptionImpl};
use commons::s2c_data_packet::S2CDataPacket;
use commons::serialization::{Serialization, SerializationImpl};

fn main() {


    // create a listener on the server information host and port
    let address = format!("{}:{}", "127.0.0.1", "8080");
    let listener = TcpListener::bind(address).unwrap();


    // print the server information
    println!(
        "server started on {}:{}",
        &"127.0.0.1", &"8080"
    );

    // for each tcp stream in incoming listeners
    for stream in listener.incoming() {

        // spawn a new thread for each new client handling the connection
        std::thread::spawn(|| {
            // unwrap the stream to get the TcpStream instance
            let mut stream = stream.unwrap();

            // print the peer address of the stream
            println!(
                "Connection established from: {}",
                // unwrap the peer address of the stream
                stream.peer_addr().unwrap()
            );

            // handle the connection e.g. packets, or messages
            let mut buffer = String::new();

            // match the string to see if it is Ok or Err
            match stream.read_to_string(&mut buffer) {
                Ok(_) => {
                    // generates a RSA keypair
                    let keys = RsaEncryptionImpl::generate_keys();

                    // assign a string the public key
                    let public_key = keys.1;

                    // Will send private key to client when AES is implemented
                    // Client needs private key to decrypt encrypted server messages!
                    let private_key = keys.0;

                    // deserialize the buffer into a C2SDataPacket type
                    let packet: C2SDataPacket = SerializationImpl.deserialize(&buffer).unwrap();

                    // Print the received message
                    println!("Received: {:?} from packet {:?}", packet.get_message(), packet);

                    // Serialize the C2S packet into a JSON string
                    let packet: String = SerializationImpl.serialize(&C2SDataPacket::new(&public_key)).unwrap();

                    // Write the response to the stream, if failed print an error message saying "Failed to send response"
                    stream.write_all(packet.as_bytes())
                        .expect("Failed to send response");
                }
                // If failed to read from the stream, print an error message saying "Failed to read from connection"
                Err(e) => {
                    println!("Failed to read from connection: {}", e);
                }
            }
        });
    }
}