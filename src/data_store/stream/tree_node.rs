use crate::error::StreamError;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

pub struct TreeNode {
    pub id: Option<TreeNodeId>,
    key: u8,
    values: Option<Vec<[String; 2]>>,
    children: HashMap<u8, Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(
        id: Option<TreeNodeId>,
        key: u8,
        values: Option<Vec<[String; 2]>>,
        children: HashMap<u8, Box<TreeNode>>,
    ) -> Self {
        Self {
            id,
            key,
            values,
            children,
        }
    }

    pub fn next(&self, key: &u8) -> Option<&Box<TreeNode>> {
        self.children.get(key)
    }

    pub fn get_child(&self, key: &u8) -> Option<&Box<TreeNode>> {
        self.children.get(key)
    }

    pub fn insert_child(
        &mut self,
        key: u8,
        mut words: TreeNodeIdIterator,
        id: TreeNodeId,
        values: Option<Vec<[String; 2]>>,
    ) {
        match words.next() {
            Some(word) => {
                let node = match self.children.get_mut(&key) {
                    Some(v) => v,
                    None => {
                        self.children.insert(
                            key,
                            Box::new(TreeNode::new(None, key, None, HashMap::new())),
                        );
                        self.children.get_mut(&key).unwrap()
                    }
                };
                node.insert_child(word, words, id, values);
            }
            None => {
                self.children.insert(
                    key,
                    Box::new(TreeNode::new(Some(id), key, values, HashMap::new())),
                );
            }
        }
    }
}

#[derive(Debug)]
pub struct TreeNodeId(pub [u64; 2]); // id[0] == bits 128 - 255

impl TreeNodeId {
    pub fn words(&self) -> TreeNodeIdIterator {
        // convert id to big endian: reorder words in each part and swap high and low parts
        let mut curr = [self[1], self[0]];
        let mut id = TreeNodeId([0, 0]);
        for _ in 0..8 {
            for i in 0..2 {
                id[i] = (id[i] << 8) | (curr[i] & 0xff);
                curr[i] >>= 8;
            }
        }
        TreeNodeIdIterator { id, ptr: 0 }
    }

    pub fn incr(&self) -> Result<TreeNodeId, Box<dyn std::error::Error>> {
        if let Some(v) = self[1].checked_add(1) {
            Ok(TreeNodeId([self[0], v]))
        } else {
            if let Some(v) = self[0].checked_add(1) {
                Ok(TreeNodeId([v, 0]))
            } else {
                Err(Box::new(StreamError::IdExhausted))
            }
        }
    }
}

impl Clone for TreeNodeId {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Deref for TreeNodeId {
    type Target = [u64; 2];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TreeNodeId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialEq for TreeNodeId {
    fn eq(&self, other: &Self) -> bool {
        self[0] == other[0] && self[1] == other[1]
    }
}

impl PartialOrd for TreeNodeId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else if self[0] < other[0] || self[0] == other[0] && self[1] < other[1] {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }
}

pub struct TreeNodeIdIterator {
    // ID in big endian
    id: TreeNodeId,
    ptr: u8,
}

impl Iterator for TreeNodeIdIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == 128 {
            return None;
        }
        self.ptr += 8;
        if self.ptr <= 64 {
            let v = (self.id[1] & 0xff) as u8;
            self.id[1] >>= 8;
            Some(v)
        } else {
            let v = (self.id[0] & 0xff) as u8;
            self.id[0] >>= 8;
            Some(v)
        }
    }
}

#[cfg(test)]
mod test {
    mod test_id_iterator {
        use crate::data_store::stream::tree_node::{TreeNodeId, TreeNodeIdIterator};

        #[test]
        fn should_return_values_in_order() {
            let words = TreeNodeIdIterator {
                id: TreeNodeId([0x0908070605040302, 0xf9f8f7f6f5f4f3f2]),
                ptr: 0,
            };
            let values = words.collect::<Vec<u8>>();
            assert_eq!(
                values,
                [0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 2, 3, 4, 5, 6, 7, 8, 9]
            );
        }
    }

    mod test_id {
        use crate::data_store::stream::tree_node::TreeNodeId;

        #[test]
        fn should_return_words_in_big_endian() {
            let id = TreeNodeId([0x0908070605040302, 0xf9f8f7f6f5f4f3f2]);
            let values = id.words().collect::<Vec<u8>>();
            assert_eq!(
                values,
                [9, 8, 7, 6, 5, 4, 3, 2, 0xf9, 0xf8, 0xf7, 0xf6, 0xf5, 0xf4, 0xf3, 0xf2]
            );
        }

        #[test]
        fn should_return_incr() {
            let id = TreeNodeId([0, 0]);
            assert_eq!(id.incr().unwrap(), TreeNodeId([0, 1]));

            let id = TreeNodeId([0, u64::MAX]);
            assert_eq!(id.incr().unwrap(), TreeNodeId([1, 0]));

            let id = TreeNodeId([u64::MAX, u64::MAX]);
            assert_eq!(
                id.incr().err().unwrap().to_string(),
                "The stream has exhausted the last possible ID, unable to add more items"
            );
        }
    }

    mod test_node {
        use std::collections::HashMap;

        use crate::data_store::stream::tree_node::{TreeNode, TreeNodeId, TreeNodeIdIterator};

        #[test]
        fn should_insert_child() {
            let id = TreeNodeId([0x0908070605040302, 0xf9f8f7f6f5f4f3f2]);
            let mut words = TreeNodeIdIterator {
                id: id.clone(),
                ptr: 0,
            };
            let key = words.next().unwrap();
            let values = vec![["foo".to_string(), "bar".to_string()]];
            let mut root = TreeNode::new(None, 0, None, HashMap::new());
            root.insert_child(key, words, id, Some(values.clone()));
            let expected_keys = [
                0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 2, 3, 4, 5, 6, 7, 8,
            ];
            let mut node = &root;
            for k in expected_keys {
                node = node.get_child(&k).unwrap();
                // All non-leaf nodes should not have any values or id
                assert_eq!(node.key, k);
                assert!(node.values.is_none());
                assert!(node.id.is_none());
                assert_eq!(node.children.len(), 1)
            }
            node = node.get_child(&9).unwrap();
            assert_eq!(node.key, 9);
            assert_eq!(node.values.as_ref().unwrap(), &values);
            assert_eq!(node.children.len(), 0);
        }
    }
}