mod connection_handler;
mod connection_handler_impl;

use crate::connection_handler::ConnectionHandler;
use std::net::TcpListener;

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

    // create a connection handler instance
    let connection_handler = connection_handler_impl::ConnectionHandlerImpl;

    // print the server information
    println!(
        "Server started on {}:{}",
        &server_information.host, &server_information.port
    );

    // for each tcp stream in incoming listeners
    for stream in listener.incoming() {
        // unwrap the stream to get the TcpStream instance
        let stream = stream.unwrap();

        // print the peer address of the stream
        println!(
            "Connection established from: {}",
            stream.peer_addr().unwrap()
        );

        // handle the connection e.g. packets, or messages
        connection_handler.handle_connection(stream);
    }
}
