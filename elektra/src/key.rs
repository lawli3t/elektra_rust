use std::collections::BTreeMap;
use std::iter::FromIterator;

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

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

pub struct KeySet {
    keys: BTreeMap<String, Key>
}

impl KeySet {
    pub fn new() -> KeySet {
        KeySet {
            keys: BTreeMap::new()
        }
    }

    pub fn size(&self) -> usize {
        self.keys.len()
    }

    pub fn append_key(&mut self, key: Key) {
        self.keys.insert(key.name().clone(), key);
    }

    pub fn append_keys(&mut self, keys: Vec<Key>)
    {
        for key in keys {
            self.append_key(key);
        }
    }

    pub fn lookup(&mut self, name: String) -> Option<Key> {
        self.keys.remove(&name)
    }

    pub fn values(&self) -> std::collections::btree_map::Iter<String, Key> {
        self.keys.iter()
    }
}

impl FromIterator<Key> for KeySet {
    fn from_iter<T: IntoIterator<Item=Key>>(iter: T) -> Self {
        let mut ks = KeySet::new();

        for key in iter {
            ks.append_key(key);
        }

        ks
    }
}