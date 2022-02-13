use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::str::FromStr;

use relative_path::{RelativePath, RelativePathBuf};

pub enum KeyNamespace {
    None,
    Cascading,
    Meta,
    Spec,
    Proc,
    Dir,
    User,
    System,
    Default,
}

pub enum KeyNamespaceError {
    InvalidNamespaceError
}

impl FromStr for KeyNamespace {
    type Err = KeyNamespaceError;

    fn from_str(namespace: &str) -> Result<Self, Self::Err> {
        match namespace {
            "meta" => Ok(KeyNamespace::Meta),
            "user" => Ok(KeyNamespace::User),
            _ => Ok(KeyNamespace::Default),
        }
    }
}

impl ToString for KeyNamespace {
    fn to_string(&self) -> String {
        let namespace = match self {
            KeyNamespace::User => "user",
            _ => "test",
        };

        namespace.to_string()
    }
}

pub struct KeyName {
    namespace: KeyNamespace,
    pub path: RelativePathBuf,
}

impl KeyName {
    pub fn new (namespace: KeyNamespace, path: RelativePathBuf) -> KeyName {
        KeyName {
            namespace,
            path
        }
    }
}

impl FromStr for KeyName {
    type Err = KeyError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        let mut splitter = name.splitn(2, ":");

        let namespace = splitter.next()
            .ok_or(KeyError::InvalidNameError)?;

        let path = splitter.next()
            .ok_or(KeyError::InvalidNameError)?;

        let key_namespace = KeyNamespace::from_str(namespace)
            .or(Err(KeyError::InvalidNameError))?;

        Ok(KeyName {
                namespace: key_namespace,
                path: RelativePathBuf::from(path).normalize(),
        })
    }
}

impl ToString for KeyName {
    fn to_string(&self) -> String {
        let mut name = self.namespace.to_string();
        name.push_str(":/");
        name.push_str(self.path.as_str());

        name
    }
}

#[derive(Debug)]
pub enum KeyError {
    InvalidNameError,
    NullPointerError,
}

type KeyValue = Vec<u8>;

pub struct Key {
    name: KeyName,
    value: Option<KeyValue>
}

impl Eq for Key {}

impl PartialEq<Self> for Key {
    fn eq(&self, other: &Self) -> bool {
        self.name.path == other.name.path
    }
}

impl PartialOrd<Self> for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.path.cmp(&other.name.path))
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.path.cmp(&other.name.path)
    }
}

impl Key {
    pub fn new(key_name: KeyName) -> Key {
        Key {
            name: key_name,
            value: None
        }
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn set_name(&mut self, name: KeyName) {
        self.name = name;
    }

    pub fn set_value(&mut self, value: KeyValue) {
        self.value = Some(value);
    }

    pub fn append_name(&mut self, name: &str) {
        self.name.path = self.name.path.join(RelativePath::new(name));
    }
}

impl FromStr for Key {
    type Err = KeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key_name = KeyName::from_str(s)?;
        Ok(Key::new(key_name))
    }
}

pub struct KeyBuilder {
    name: KeyName,
    value: Option<KeyValue>
}

impl KeyBuilder {
    pub fn new(namespace: KeyNamespace, path: RelativePathBuf) -> KeyBuilder {
        KeyBuilder {
            name: KeyName::new(namespace, path),
            value: None,
        }
    }

    pub fn value(mut self, value: KeyValue) -> KeyBuilder {
        self.value = Some(value);
        self
    }

    pub fn build(self) -> Result<Key, KeyError> {
        let mut key = Key::new(self.name);

        if let Some(value) = self.value {
            key.set_value(value);
        }

        Ok(key)
    }
}

impl FromStr for KeyBuilder {
    type Err = KeyError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        let key_name = KeyName::from_str(name)?;

        Ok(KeyBuilder {
            name: key_name,
            value: None,
        })
    }
}

pub struct KeySet {
    keys: BTreeMap<String, Key>
}

impl KeySet {
    pub fn size(&self) -> usize {
        self.keys.len()
    }

    pub fn append_key(&mut self, key: Key) {
        self.keys.insert(key.name().clone(), key);
    }

    /*
    pub fn append_keys(&mut self, keys: &[Key])
    {
        for key in keys {
            self.append_key(key);
        }
    }
    */

    pub fn lookup(&mut self, name: String) -> Option<Key> {
        self.keys.remove(&name)
    }

    pub fn values(&self) -> std::collections::btree_map::Iter<String, Key> {
        self.keys.iter()
    }
}

impl Default for KeySet {
    fn default() -> KeySet {
        KeySet {
            keys: BTreeMap::new()
        }
    }
}

impl FromIterator<Key> for KeySet {
    fn from_iter<T: IntoIterator<Item=Key>>(iter: T) -> Self {
        let mut ks = KeySet::default();

        for key in iter {
            ks.append_key(key);
        }

        ks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key() {
        let key = Key::new(
            KeyName::from_str("user:/test/qwe/asd").unwrap()
        );

        assert_eq!(key.name(), "user:/test/qwe/asd")
    }

    #[test]
    fn test_key_builder() {
        let key = KeyBuilder::from_str("user:/test/qwe/asd")
            .unwrap()
            .value([1, 2, 3].to_vec())
            .build()
            .unwrap();

        assert_eq!(key.name(), "user:/test/qwe/asd");
    }
}