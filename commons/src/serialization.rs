use serde::{Deserialize, Serialize};

pub trait Serialization {
    fn serialize<T>(&self, data: &T) -> Result<String, serde_json::Error>
    where
        T: Serialize;

    fn deserialize<'a, T>(&self, data: &'a str) -> Result<T, serde_json::Error>
    where
        T: Deserialize<'a>;
}

pub struct SerializationImpl;

impl Serialization for SerializationImpl {
    fn serialize<T>(&self, data: &T) -> Result<String, serde_json::Error>
    where T: Serialize,
    {
        serde_json::to_string(data)
    }

    fn deserialize<'a, T>(&self, data: &'a str) -> Result<T, serde_json::Error>
    where T: Deserialize<'a>,
    {
        serde_json::from_str(data)
    }
}