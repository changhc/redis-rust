use std::collections::HashMap;

use super::tree_node::{TreeNode, TreeNodeId};
use crate::error::StreamError;

pub struct RadixTree {
    root: Box<TreeNode>,
    top_id: TreeNodeId,
}

impl RadixTree {
    pub fn new() -> Self {
        let top_id = TreeNodeId([0, 0]);
        RadixTree {
            root: Box::new(TreeNode::new(None, 0, None, HashMap::new())),
            top_id,
        }
    }

    pub fn insert(
        &mut self,
        id: Option<[u64; 2]>,
        values: Vec<[String; 2]>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let new_id = match id {
            Some(v) => TreeNodeId(v),
            None => match self.top_id.incr() {
                Ok(v) => v,
                Err(_) => return Err(Box::new(StreamError::IdExhausted)),
            },
        };
        if new_id <= self.top_id {
            return Err(Box::new(StreamError::IdNotGreaterThanStreamTop));
        }

        let mut words = new_id.words();
        let word = words.next().unwrap();
        self.root
            .insert_child(word, words, new_id.clone(), Some(values))?;
        self.top_id = new_id;
        Ok(())
    }

    pub fn remove(&mut self, id: [u64; 2]) {
        let id = TreeNodeId(id);
        self.root.remove_child(id.words());
    }
}
