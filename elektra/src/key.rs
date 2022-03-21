use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::str::FromStr;

use relative_path::{RelativePath, RelativePathBuf};

#[derive(Copy, Clone, PartialEq, Debug)]
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
            KeyNamespace::Meta => "meta",
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

    pub fn base_name(&self) -> Option<&str> {
        self.path.file_name()
    }

    pub fn set_base_name(&mut self, base_name: &str) {
        self.path.set_file_name(base_name);
    }

    pub fn append_name(&mut self, name: &str) {
        self.path = self.path.join(RelativePath::new(name));
    }

    pub fn namespace(&self) -> KeyNamespace {
        self.namespace
    }

    pub fn set_namespace(&mut self, namespace: KeyNamespace) {
        self.namespace = namespace
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

        Ok(KeyName::new(
            key_namespace,
            RelativePathBuf::from(path).normalize(),
        ))
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

pub struct Key {
    name: KeyName,
    value: Option<Vec<u8>>
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

    pub fn name(&self) -> &KeyName {
        &self.name
    }

    pub fn name_mut(&mut self) -> &mut KeyName {
        &mut self.name
    }

    pub fn set_name(&mut self, name: KeyName) {
        self.name = name;
    }

    pub fn set_value(&mut self, value: &[u8]) {
        self.value = Some(value.to_vec());
    }

    pub fn set_value_str(&mut self, value: &str) {
        self.value = Some(value.as_bytes().to_vec())
    }

    pub fn value(&self) -> Option<&[u8]> {
        match &self.value {
            Some(value) => Some(value.as_slice()),
            None => None,
        }
    }

    pub fn value_string(&self) -> Option<String> {
        if let Some(value) = &self.value {
            Some(String::from_utf8_lossy(value).to_string())
        } else {
            None
        }
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
    value: Option<Vec<u8>>
}

impl KeyBuilder {
    pub fn new(key_name: KeyName) -> KeyBuilder {
        KeyBuilder {
            name: key_name,
            value: None,
        }
    }

    pub fn value(mut self, value: &[u8]) -> KeyBuilder {
        self.value = Some(value.to_vec());
        self
    }

    pub fn build(self) -> Result<Key, KeyError> {
        let mut key = Key::new(self.name);

        if let Some(value) = self.value {
            key.set_value(&value);
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
        self.keys.insert(key.name().to_string().clone(), key);
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

        assert_eq!(key.name().to_string(), "user:/test/qwe/asd");
    }

    #[test]
    fn test_key_name() {
        let mut key_name = KeyName::from_str("user:/test/qwe/asd").unwrap();
        assert_eq!(key_name.to_string(), "user:/test/qwe/asd");
        assert_eq!(key_name.base_name().unwrap(), "asd");
        assert_eq!(key_name.namespace(), KeyNamespace::User);

        key_name.append_name("qweqweqwe");
        assert_eq!(key_name.to_string(), "user:/test/qwe/asd/qweqweqwe");
        assert_eq!(key_name.base_name().unwrap(), "qweqweqwe");
        assert_eq!(key_name.namespace(), KeyNamespace::User);

        key_name.set_base_name("zxc");
        assert_eq!(key_name.to_string(), "user:/test/qwe/asd/zxc");
        assert_eq!(key_name.base_name().unwrap(), "zxc");

        key_name.set_namespace(KeyNamespace::Meta);
        assert_eq!(key_name.to_string(), "meta:/test/qwe/asd/zxc");
        assert_eq!(key_name.namespace(), KeyNamespace::Meta);

        key_name.set_namespace(KeyNamespace::User);
        assert_eq!(key_name.to_string(), "user:/test/qwe/asd/zxc");
        assert_eq!(key_name.namespace(), KeyNamespace::User);
    }

    #[test]
    fn test_key_value() {
        let mut key = Key::new(
            KeyName::from_str("user:/test").unwrap()
        );

        assert_eq!(None, key.value());
        assert_eq!(None, key.value_string());

        key.set_value_str("asdf");
        assert_eq!(vec![97, 115, 100, 102], key.value().unwrap());
        assert_eq!("asdf".as_bytes(), key.value().unwrap());
        assert_eq!("asdf", key.value_string().unwrap());
    }

    #[test]
    fn test_key_builder() {
        let key = KeyBuilder::from_str("user:/test/qwe/asd")
            .unwrap()
            .value("asd".as_bytes())
            .build()
            .unwrap();

        assert_eq!(key.name().to_string(), "user:/test/qwe/asd");
        assert_eq!(key.value_string().unwrap(), "asd");
    }
}