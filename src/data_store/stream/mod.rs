mod radix_tree;
mod tree_node;

use crate::error::StreamError;
use radix_tree::RadixTree;
use tree_node::TreeNodeId;

pub struct Stream {
    tree: RadixTree,
}

impl Stream {
    pub fn new() -> Self {
        Stream {
            tree: RadixTree::new(),
        }
    }

    pub fn insert(
        &mut self,
        id: Option<[u64; 2]>,
        values: Vec<[String; 2]>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let new_id = match id {
            Some(v) => TreeNodeId(v),
            None => match self.tree.next_available_id(None) {
                Ok(v) => v,
                Err(_) => return Err(Box::new(StreamError::IdExhausted)),
            },
        };

        if new_id <= *self.tree.top_id() {
            return Err(Box::new(StreamError::IdNotGreaterThanStreamTop));
        }

        self.tree.insert(new_id, values)
    }
}
