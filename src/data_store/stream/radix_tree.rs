use std::collections::HashMap;

use super::tree_node::{TreeNode, TreeNodeId};

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

    pub fn next_available_id(&self, higher_part: Option<u64>) -> Result<TreeNodeId, ()> {
        match higher_part {
            Some(v) => {
                let id = TreeNodeId([v, 0]);
                let mut words = id.words();
                let mut node = self.root.as_ref();
                for _ in 0..8 {
                    let key = words.next().unwrap();
                    match node.get_child(&key) {
                        Some(n) => node = n,
                        None => return Ok(id),
                    }
                }
                let greatest_node = node.get_greatest_child();
                greatest_node.get_id().unwrap().incr()
            }
            None => self.top_id.incr(),
        }
    }

    pub fn top_id(&self) -> &TreeNodeId {
        &self.top_id
    }

    pub fn insert(
        &mut self,
        new_id: TreeNodeId,
        values: Vec<[String; 2]>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut words = new_id.words();
        let word = words.next().unwrap();
        self.root
            .insert_child(word, words, new_id.clone(), Some(values))?;
        self.top_id = new_id;
        Ok(self.top_id.to_string())
    }

    pub fn remove(&mut self, id: [u64; 2]) {
        let id = TreeNodeId(id);
        self.root.remove_child(id.words());
    }
}
