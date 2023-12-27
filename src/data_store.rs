use std::collections::{HashMap, LinkedList};

pub type DataStore = HashMap<String, RedisEntry>;

pub enum RedisEntryType {
    String,
    List,
}

pub struct RedisEntry {
    pub type_: RedisEntryType,
    pub string: Option<String>,
    pub list: Option<LinkedList<String>>,
}

impl RedisEntry {
    pub fn create_string(value: &String) -> Self {
        RedisEntry {
            type_: RedisEntryType::String,
            string: Some(value.clone()),
            list: None,
        }
    }

    pub fn init_list() -> Self {
        RedisEntry {
            type_: RedisEntryType::List,
            string: None,
            list: Some(LinkedList::<String>::new()),
        }
    }
}

pub fn get_data_store() -> DataStore {
    DataStore::new()
}

#[cfg(test)]
mod test {
    use super::{get_data_store, RedisEntry};

    #[test]
    fn test_list_store() {
        let mut ds = get_data_store();
        let s = RedisEntry::init_list();
        ds.insert("foo".to_string(), s);

        let v = ds.get_mut(&"foo".to_string()).unwrap();
        v.list.as_mut().unwrap().push_back("aaa".to_string());
        v.list.as_mut().unwrap().push_front("bbb".to_string());

        let v = ds.get(&"foo".to_string()).unwrap();
        assert_eq!(v.list.as_ref().unwrap().len(), 2);
        assert_eq!(v.list.as_ref().unwrap().back().unwrap(), &"aaa".to_string());
    }
}
