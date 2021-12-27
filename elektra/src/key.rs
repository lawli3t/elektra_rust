pub struct Key {
    name: String,
}

impl Key {
    pub fn new(name: String) -> Key {
        Key {
            name
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
}
