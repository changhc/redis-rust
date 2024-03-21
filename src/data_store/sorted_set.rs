use rand::Rng;
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};

const SKIP_LIST_MAX_LEVEL: u8 = 32;
const SKIP_LIST_PROB: f64 = 0.5;

struct ListNode {
    id: u64,
    level: u8,
    next: HashMap<u8, u64>,
    score: f64,
    values: BTreeSet<String>,
}

impl ListNode {
    pub fn new(id: u64, level: u8, score: f64) -> Self {
        ListNode {
            id,
            level,
            score,
            next: HashMap::new(),
            values: BTreeSet::new(),
        }
    }

    pub fn set_level(&mut self, level: u8) {
        self.level = level;
    }

    pub fn add_value(&mut self, value: String) {
        self.values.insert(value);
    }

    pub fn set_next(&mut self, level: u8, node: &RefCell<ListNode>) {
        self.next.insert(level, node.borrow().id);
    }

    pub fn get_next(&self, level: u8) -> Option<u64> {
        self.next.get(&level).cloned()
    }
}

pub struct SkipList {
    max_level: u8,
    prob: f64,
    head_id: u64,
    nodes: HashMap<u64, RefCell<ListNode>>,
    next_node_id: u64,
}

impl Default for SkipList {
    fn default() -> Self {
        Self::new(SKIP_LIST_MAX_LEVEL)
    }
}

impl SkipList {
    pub fn new(level: u8) -> Self {
        let max_level = level;
        let head_node = RefCell::new(ListNode::new(0, max_level, f64::MIN));
        let head_id = head_node.borrow().id;
        let tail_node = RefCell::new(ListNode::new(1, max_level, f64::MAX));
        let tail_id = tail_node.borrow().id;
        for level in 0..=max_level {
            head_node.borrow_mut().set_next(level, &tail_node);
        }
        let nodes = HashMap::from([(head_id, head_node), (tail_id, tail_node)]);
        SkipList {
            max_level,
            prob: SKIP_LIST_PROB,
            head_id,
            nodes,
            next_node_id: 2,
        }
    }

    fn should_insert(&self, score: f64, value: &str) -> Option<Vec<(u8, u64)>> {
        let mut level: i16 = self.max_level as i16;
        let mut current_node_id = self.head_id;
        // List of the immediate previous node per level. (level, node_id)
        let mut previous_nodes = Vec::<(u8, u64)>::new();
        while level >= 0 {
            let current_node = self.nodes.get(&current_node_id).unwrap();
            let current_node_score = current_node.borrow().score;
            let next_node_id = current_node.borrow().get_next(level as u8).unwrap();
            let next_node = self.nodes.get(&next_node_id).unwrap();
            let next_node_score = next_node.borrow().score;
            if score == current_node_score {
                current_node.borrow_mut().add_value(value.to_owned());
                return None;
            } else if score >= next_node_score {
                current_node_id = next_node_id;
            } else {
                previous_nodes.push((level as u8, current_node.borrow().id));
                level -= 1;
            }
        }
        Some(previous_nodes)
    }

    fn insert_node_at_level(
        &self,
        current_node: &RefCell<ListNode>,
        new_node: &RefCell<ListNode>,
        current_level: u8,
    ) {
        let next_next_id_op = current_node.borrow().get_next(current_level);
        if let Some(next_next_id) = next_next_id_op {
            let next_node = self.nodes.get(&next_next_id).unwrap();
            new_node.borrow_mut().set_next(current_level, next_node);
        }
        new_node.borrow_mut().set_level(current_level);
        current_node.borrow_mut().set_next(current_level, new_node);
    }

    fn create_new_node(&mut self, score: f64, value: &str) -> u64 {
        let new_node_id = self.next_node_id;
        let new_node = RefCell::new(ListNode::new(new_node_id, 0, score));
        new_node.borrow_mut().add_value(value.to_owned());
        self.nodes.insert(new_node_id, new_node);
        self.next_node_id += 1;
        new_node_id
    }

    pub fn insert(&mut self, score: f64, value: String) {
        let prev_op = self.should_insert(score, &value);
        if prev_op.is_none() {
            return;
        }
        let mut previous_nodes = prev_op.unwrap();

        let new_node_id = self.create_new_node(score, &value);
        let new_node = self.nodes.get(&new_node_id).unwrap();
        let (current_level, current_node_id) = previous_nodes.pop().unwrap();
        assert_eq!(current_level, 0);
        let current_node = self.nodes.get(&current_node_id).unwrap();
        self.insert_node_at_level(current_node, new_node, current_level);

        let mut rng = rand::thread_rng();
        while rng.gen::<f64>() > self.prob {
            if let Some((current_level, current_node_id)) = previous_nodes.pop() {
                let current_node = self.nodes.get(&current_node_id).unwrap();
                self.insert_node_at_level(current_node, new_node, current_level);
            } else {
                break;
            }
        }
    }

    pub fn get_values(&self, start_score: f64, stop_score: f64) -> Vec<String> {
        let mut result = Vec::new();
        let mut level: i16 = self.max_level as i16;
        let mut current_node_id = self.head_id;
        while level >= 0 {
            let current_node = self.nodes.get(&current_node_id).unwrap();
            let next_node_id = current_node.borrow().get_next(level as u8).unwrap();
            let next_node = self.nodes.get(&next_node_id).unwrap();
            let next_node_score = next_node.borrow().score;
            if start_score == current_node.borrow().score {
                break;
            } else if start_score >= next_node_score {
                current_node_id = next_node_id;
            } else {
                level -= 1;
            }
        }
        let mut current_node = self.nodes.get(&current_node_id).unwrap();
        let current_node_score = current_node.borrow().score;
        if start_score > current_node_score {
            let next_node_id = current_node.borrow().get_next(0).unwrap();
            let next_node = self.nodes.get(&next_node_id).unwrap();
            current_node = next_node;
        }

        let mut current_node_score = current_node.borrow().score;
        while current_node_score <= stop_score {
            for v in current_node.borrow().values.iter() {
                result.push(v.to_owned());
            }
            let next_node_id = current_node.borrow().get_next(0).unwrap();
            current_node = self.nodes.get(&next_node_id).unwrap();
            current_node_score = current_node.borrow().score;
        }
        result
    }
}

#[cfg(test)]
mod test {
    mod test_listnode {
        use crate::data_store::sorted_set::ListNode;
        use std::cell::RefCell;

        #[test]
        fn should_set_next_nodes() {
            let n0 = RefCell::new(ListNode::new(0, 0, 1.0));
            let n1 = RefCell::new(ListNode::new(1, 0, 1.0));
            n0.borrow_mut().set_next(0, &n1);
            assert_eq!(n0.borrow().get_next(0).unwrap(), 1);
        }

        #[test]
        fn should_set_values_and_return_sorted_values() {
            let n0 = RefCell::new(ListNode::new(0, 0, 1.0));
            n0.borrow_mut().add_value("c".to_string());
            n0.borrow_mut().add_value("b".to_string());
            n0.borrow_mut().add_value("a".to_string());
            n0.borrow_mut().add_value("aa".to_string());
            assert_eq!(
                n0.borrow().values.iter().collect::<Vec<_>>(),
                [
                    &"a".to_string(),
                    &"aa".to_string(),
                    &"b".to_string(),
                    &"c".to_string()
                ]
            );
        }
    }

    mod test_skiplist {
        use crate::data_store::sorted_set::SkipList;

        #[test]
        fn should_insert_node() {
            let mut list = SkipList::new(2);
            // set prob to -1 so that nodes are always created in order to remove randomness
            list.prob = -1.0;

            list.insert(1.0, "foo".to_string());
            list.insert(3.0, "bar".to_string());
            list.insert(2.0, "baz".to_string());
            list.insert(1.0, "foobar".to_string());
            let mut nodes = list.nodes.values().collect::<Vec<_>>();
            nodes.sort_by(|a, b| a.borrow().id.cmp(&b.borrow().id));
            assert_eq!(nodes.len(), 5);
            let expected = [
                (f64::MIN, vec![]),
                (f64::MAX, vec![]),
                (1.0, vec!["foo", "foobar"]),
                (3.0, vec!["bar"]),
                (2.0, vec!["baz"]),
            ];
            for i in 0..nodes.len() {
                assert_eq!(nodes[i].borrow().score, expected[i].0);
                assert_eq!(
                    nodes[i].borrow().values.iter().collect::<Vec<_>>(),
                    expected[i].1
                );
            }
        }

        #[test]
        fn insert_node_should_return_none_when_scores_exist() {
            let mut list = SkipList::new(2);
            // set prob to -1 so that nodes are always created in order to remove randomness
            list.prob = -1.0;

            assert_eq!(
                list.should_insert(1.0, "a").unwrap(),
                [(2, 0), (1, 0), (0, 0)]
            );
            list.insert(1.0, "a".to_string());
            assert!(list.should_insert(1.0, "b").is_none());
            assert_eq!(
                list.should_insert(3.0, "c").unwrap(),
                [(2, 2), (1, 2), (0, 2)]
            );
            assert_eq!(
                list.should_insert(2.0, "d").unwrap(),
                [(2, 2), (1, 2), (0, 2)]
            );
        }

        #[test]
        fn should_create_new_node() {
            let mut list = SkipList::new(2);
            assert_eq!(list.next_node_id, 2);
            let node_id = list.create_new_node(1.0, "foo");
            assert_eq!(node_id, 2);
            assert_eq!(list.next_node_id, 3);
        }

        #[test]
        fn should_insert_node_at_level() {
            let mut list = SkipList::new(2);
            list.create_new_node(1.0, "a");
            list.create_new_node(3.0, "b");
            list.create_new_node(2.0, "c");
            let n0 = list.nodes.get(&2).unwrap();
            let n1 = list.nodes.get(&3).unwrap();
            let n2 = list.nodes.get(&4).unwrap();
            n0.borrow_mut().set_next(0, n1);
            list.insert_node_at_level(n0, n2, 0);
            assert_eq!(n0.borrow().get_next(0).unwrap(), 4);
        }

        #[test]
        fn should_get_all_values() {
            let mut list = SkipList::new(2);
            let input = [(1.0, "a"), (3.0, "b"), (2.0, "c"), (1.0, "d"), (3.9, "e")];
            for (score, value) in input {
                list.insert(score, value.to_string());
            }
            let values = list.get_values(-1.0, 4.0);
            assert_eq!(values, ["a", "d", "c", "b", "e"]);
            let values = list.get_values(1.5, 4.0);
            assert_eq!(values, ["c", "b", "e"]);
            let values = list.get_values(1.5, 3.5);
            assert_eq!(values, ["c", "b"]);
            let values = list.get_values(1.5, 1.9);
            assert!(values.is_empty());
            let values = list.get_values(2.0, 1.9);
            assert!(values.is_empty());
        }
    }
}
