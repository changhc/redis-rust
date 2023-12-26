use std::collections::{HashMap, LinkedList};

pub struct DataStore {
    string: HashMap<String, String>,
    list: HashMap<String, RedisList>,
}

impl DataStore {
    pub fn new() -> Self {
        DataStore {
            string: HashMap::<String, String>::new(),
            list: HashMap::<String, RedisList>::new(),
        }
    }

    pub fn get_string_store(&mut self) -> &mut HashMap<String, String> {
        &mut self.string
    }

    pub fn get_list_store(&mut self) -> &mut HashMap<String, RedisList> {
        &mut self.list
    }
}

pub struct RedisList {
    length: usize,
    items: LinkedList<String>,
}

impl RedisList {
    pub fn new() -> Self {
        RedisList {
            length: 0,
            items: LinkedList::<String>::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{DataStore, RedisList};

    #[test]
    fn test_list_store() {
        let mut ds = DataStore::new();
        let s = ds.get_list_store();
        s.insert("foo".to_string(), RedisList::new());

        let v = s.get_mut(&"foo".to_string()).unwrap();
        v.items.push_back("aaa".to_string());
        v.items.push_front("bbb".to_string());
        v.length += 2;

        let v = s.get(&"foo".to_string()).unwrap();
        assert_eq!(v.length, 2);
        assert_eq!(v.items.back().unwrap(), &"aaa".to_string());
    }
}
