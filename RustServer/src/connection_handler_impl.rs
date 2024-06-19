use std::io::{Read, Write};
use std::net::TcpStream;

use crate::connection_handler::ConnectionHandler;

pub struct ConnectionHandlerImpl;

// Implement the ConnectionHandler trait for ConnectionHandlerImpl
impl ConnectionHandler for ConnectionHandlerImpl {
    // Implement the handle_connection method for ConnectionHandlerImpl
    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = String::new();

        // match the string to see if it is Ok or Err
        match stream.read_to_string(&mut buffer) {
            Ok(_) => {
                // Print the received message
                println!("Received: {}", buffer);
                // Send a response back to the client
                let response = "Message received! Hello Client!";

                // Write the response to the stream, if failed print an error message saying "Failed to send response"
                stream
                    .write_all(response.as_bytes())
                    .expect("Failed to send response");
            }
            // If failed to read from the stream, print an error message saying "Failed to read from connection"
            Err(e) => {
                println!("Failed to read from connection: {}", e);
            }
        }
    }
}
