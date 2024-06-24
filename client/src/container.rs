pub struct ContainerImpl {
    key: String,
}

pub trait Container {
    fn new(key: String) -> Self;

    fn get_key(&self) -> String;
}

impl Container for ContainerImpl {
    fn new(key: String) -> Self {
        ContainerImpl {
            key,
        }
    }

    fn get_key(&self) -> String {
        return self.key.to_string();
    }
}