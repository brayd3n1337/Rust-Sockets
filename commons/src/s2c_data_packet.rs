use serde::{Deserialize, Serialize};
pub use crate::packet::Packet;

// allow JSON serialization and deserialization for this class
#[derive(Serialize, Deserialize, Debug)]
pub struct S2CDataPacket {
    // message
    pub data: String,

}

// implement the Packet trait for the C2SDataPacket struct
impl Packet for S2CDataPacket {

    // constructor
    fn new(message: &str) -> Self {

        // create new instance of C2SDataPacket struct
        S2CDataPacket {
            // define the data field in the struct as the message
            data: message.to_string(),
        }
    }

    // return the message
    fn get_message(&self) -> String {
        return self.data.to_string();
        //     ^^ self is the same as this in java or c++
    }
}