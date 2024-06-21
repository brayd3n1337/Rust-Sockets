pub trait Packet {
    fn new(message: &str) -> Self;

    fn get_message(&self) -> String;
}

pub struct PacketImpl {
    message: String,
}

impl Packet for PacketImpl {
    fn new(message: &str) -> Self {
        PacketImpl {
            message: message.to_string(),
        }
    }

    fn get_message(&self) -> String {
        return self.message.to_string();
    }
}