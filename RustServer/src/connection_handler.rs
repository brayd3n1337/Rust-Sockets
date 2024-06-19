// src/connection_handler.rs

use std::net::TcpStream;

pub trait ConnectionHandler {
    fn handle_connection(&self, stream: TcpStream);
}
