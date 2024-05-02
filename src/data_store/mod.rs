mod sorted_set;
mod stream;

use crate::error::{ExecutionError, InternalError};

use sorted_set::SortedSet;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fmt::{Display, Formatter};

use self::stream::Stream;

pub struct DataStore {
    ds: HashMap<String, RedisEntry>,
}

impl DataStore {
    pub fn new() -> Self {
        Self { ds: HashMap::new() }
    }

    pub fn get_string(&self, key: &String) -> Result<Option<&String>, Box<dyn std::error::Error>> {
        match self.ds.get(key) {
            Some(entry) => match entry.type_ {
                RedisEntryType::String => match &entry.string {
                    Some(v) => Ok(Some(v)),
                    None => Err(Self::throw_integration_error(key, RedisEntryType::String)),
                },
                _ => Err(Box::new(ExecutionError::IncorrectType)),
            },
            None => Ok(None),
        }
    }

    pub fn set_string_overwrite(&mut self, key: &str, value: &str) {
        self.ds
            .insert(key.to_owned(), RedisEntry::create_string(value));
    }

    pub fn set_string(
        &mut self,
        key: &String,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self.ds.get_mut(key) {
            Some(entry) => match entry.type_ {
                RedisEntryType::String => match &entry.string {
                    Some(_) => {
                        entry.string = Some(value.to_owned());
                        Ok(())
                    }
                    None => Err(Self::throw_integration_error(key, RedisEntryType::String)),
                },
                _ => Err(Box::new(ExecutionError::IncorrectType)),
            },
            None => {
                self.set_string_overwrite(key, value);
                Ok(())
            }
        }
    }

    pub fn get_list_mut(
        &mut self,
        key: &String,
    ) -> Result<Option<&mut LinkedList<String>>, Box<dyn std::error::Error>> {
        match self.ds.get_mut(key) {
            Some(entry) => match entry.type_ {
                RedisEntryType::List => match &mut entry.list {
                    Some(v) => Ok(Some(v)),
                    None => Err(Self::throw_integration_error(key, RedisEntryType::List)),
                },
                _ => Err(Box::new(ExecutionError::IncorrectType)),
            },
            None => Ok(None),
        }
    }

    pub fn insert_list(&mut self, key: &String) -> Result<(), Box<dyn std::error::Error>> {
        match self.ds.get(key) {
            Some(_) => Err(Box::new(InternalError::KeyError)),
            None => {
                let s = RedisEntry::init_list();
                self.ds.insert(key.clone(), s);
                Ok(())
            }
        }
    }

    pub fn get_set_mut(
        &mut self,
        key: &String,
    ) -> Result<Option<&mut HashSet<String>>, Box<dyn std::error::Error>> {
        match self.ds.get_mut(key) {
            Some(entry) => match entry.type_ {
                RedisEntryType::Set => match &mut entry.set {
                    Some(v) => Ok(Some(v)),
                    None => Err(Self::throw_integration_error(key, RedisEntryType::Set)),
                },
                _ => Err(Box::new(ExecutionError::IncorrectType)),
            },
            None => Ok(None),
        }
    }

    pub fn insert_set(&mut self, key: &String) -> Result<(), Box<dyn std::error::Error>> {
        match self.ds.get(key) {
            Some(_) => Err(Box::new(InternalError::KeyError)),
            None => {
                let s = RedisEntry::init_set();
                self.ds.insert(key.clone(), s);
                Ok(())
            }
        }
    }

    pub fn get_hash_mut(
        &mut self,
        key: &String,
    ) -> Result<Option<&mut HashMap<String, String>>, Box<dyn std::error::Error>> {
        match self.ds.get_mut(key) {
            Some(entry) => match entry.type_ {
                RedisEntryType::Hash => match &mut entry.hash {
                    Some(v) => Ok(Some(v)),
                    None => Err(Self::throw_integration_error(key, RedisEntryType::Hash)),
                },
                _ => Err(Box::new(ExecutionError::IncorrectType)),
            },
            None => Ok(None),
        }
    }

    pub fn insert_hash(&mut self, key: &String) -> Result<(), Box<dyn std::error::Error>> {
        match self.ds.get(key) {
            Some(_) => Err(Box::new(InternalError::KeyError)),
            None => {
                let s = RedisEntry::init_hash();
                self.ds.insert(key.clone(), s);
                Ok(())
            }
        }
    }

    pub fn get_sorted_set_mut(
        &mut self,
        key: &String,
    ) -> Result<Option<&mut SortedSet>, Box<dyn std::error::Error>> {
        match self.ds.get_mut(key) {
            Some(entry) => match entry.type_ {
                RedisEntryType::SortedSet => match &mut entry.sorted_set {
                    Some(v) => Ok(Some(v)),
                    None => Err(Self::throw_integration_error(
                        key,
                        RedisEntryType::SortedSet,
                    )),
                },
                _ => Err(Box::new(ExecutionError::IncorrectType)),
            },
            None => Ok(None),
        }
    }

    pub fn insert_sorted_set(&mut self, key: &String) -> Result<(), Box<dyn std::error::Error>> {
        match self.ds.get(key) {
            Some(_) => Err(Box::new(InternalError::KeyError)),
            None => {
                let s = RedisEntry::init_sorted_set();
                self.ds.insert(key.clone(), s);
                Ok(())
            }
        }
    }

    pub fn get_stream_mut(
        &mut self,
        key: &String,
    ) -> Result<Option<&mut Stream>, Box<dyn std::error::Error>> {
        match self.ds.get_mut(key) {
            Some(entry) => match entry.type_ {
                RedisEntryType::Stream => match &mut entry.stream {
                    Some(v) => Ok(Some(v)),
                    None => Err(Self::throw_integration_error(key, RedisEntryType::Stream)),
                },
                _ => Err(Box::new(ExecutionError::IncorrectType)),
            },
            None => Ok(None),
        }
    }

    pub fn insert_stream(&mut self, key: &String) -> Result<(), Box<dyn std::error::Error>> {
        match self.ds.get(key) {
            Some(_) => Err(Box::new(InternalError::KeyError)),
            None => {
                let s = RedisEntry::init_stream();
                self.ds.insert(key.clone(), s);
                Ok(())
            }
        }
    }

    pub fn drop_key(&mut self, key: &String) {
        self.ds.remove(key);
    }

    fn throw_integration_error(
        key: &String,
        expected_type: RedisEntryType,
    ) -> Box<dyn std::error::Error> {
        log::error!(
            "Integration error at key '{}': expecting type '{}' but data is not found",
            key,
            expected_type.to_string()
        );
        Box::new(InternalError::Error)
    }
}

impl Default for DataStore {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default, Debug)]
pub enum RedisEntryType {
    #[default]
    Unknown,
    String,
    List,
    Set,
    Hash,
    SortedSet,
    Stream,
}

impl Display for RedisEntryType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default)]
pub struct RedisEntry {
    pub type_: RedisEntryType,
    pub string: Option<String>,
    pub list: Option<LinkedList<String>>,
    pub set: Option<HashSet<String>>,
    pub hash: Option<HashMap<String, String>>,
    pub sorted_set: Option<SortedSet>,
    pub stream: Option<Stream>,
}

impl RedisEntry {
    pub fn create_string(value: &str) -> Self {
        RedisEntry {
            type_: RedisEntryType::String,
            string: Some(value.to_owned()),
            ..Default::default()
        }
    }

    pub fn init_list() -> Self {
        RedisEntry {
            type_: RedisEntryType::List,
            list: Some(LinkedList::new()),
            ..Default::default()
        }
    }

    pub fn init_set() -> Self {
        RedisEntry {
            type_: RedisEntryType::Set,
            set: Some(HashSet::new()),
            ..Default::default()
        }
    }

    pub fn init_hash() -> Self {
        RedisEntry {
            type_: RedisEntryType::Hash,
            hash: Some(HashMap::new()),
            ..Default::default()
        }
    }

    pub fn init_sorted_set() -> Self {
        RedisEntry {
            type_: RedisEntryType::SortedSet,
            sorted_set: Some(SortedSet::new()),
            ..Default::default()
        }
    }

    pub fn init_stream() -> Self {
        RedisEntry {
            type_: RedisEntryType::Stream,
            stream: Some(Stream::new()),
            ..Default::default()
        }
    }
}

pub fn get_data_store() -> DataStore {
    DataStore::new()
}

#[cfg(test)]
mod test {
    use super::get_data_store;

    #[test]
    fn test_list_store() {
        let mut ds = get_data_store();
        let key = "foo".to_string();
        let _ = ds.insert_list(&key);

        let v = ds.get_list_mut(&key).unwrap().unwrap();
        v.push_front("aaa".to_string());
        v.push_front("bbb".to_string());

        let v = ds.ds.get(&"foo".to_string()).unwrap();
        assert_eq!(v.list.as_ref().unwrap().len(), 2);
        assert_eq!(v.list.as_ref().unwrap().back().unwrap(), &"aaa".to_string());
    }
}
