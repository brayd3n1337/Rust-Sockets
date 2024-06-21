use std::io::{Read, Write};
use std::net::TcpListener;
use commons::c2s_data_packet::C2SDataPacket;
use commons::packet::Packet;
use commons::serialization::{Serialization, SerializationImpl};

struct ServerInformation {
    host: String,
    port: u16,
}

fn main() {
    // create a server information struct
    let server_information = ServerInformation {
        host: "127.0.0.1".to_string(),
        port: 8080,
    };

    // create a listener on the server information host and port
    let address = format!("{}:{}", server_information.host, server_information.port);
    let listener = TcpListener::bind(address).unwrap();

    // print the server information
    println!(
        "server started on {}:{}",
        &server_information.host, &server_information.port
    );

    // for each tcp stream in incoming listeners
    for stream in listener.incoming() {
        // unwrap the stream to get the TcpStream instance
        let mut stream = stream.unwrap();

        // print the peer address of the stream
        println!(
            "Connection established from: {}",
            stream.peer_addr().unwrap()
        );

        // handle the connection e.g. packets, or messages
        let mut buffer = String::new();

        let mut serialization = SerializationImpl;

        // match the string to see if it is Ok or Err
        match stream.read_to_string(&mut buffer) {
            Ok(_) => {

                // deserialize the buffer into a C2SDataPacket type
                let packet: C2SDataPacket = serialization.deserialize(&buffer).unwrap();

                // Print the received message
                println!("Received: {:?} from packet {:?}", packet.get_message(), packet);

                // Send a response back to the client
                let response = "Message received! Hello client!";

                // Write the response to the stream, if failed print an error message saying "Failed to send response"
                stream.write_all(response.as_bytes())
                    .expect("Failed to send response");
            }
            // If failed to read from the stream, print an error message saying "Failed to read from connection"
            Err(e) => {
                println!("Failed to read from connection: {}", e);
            }
        }
    }
}