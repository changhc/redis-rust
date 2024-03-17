use rand::Rng;
use std::cell::RefCell;
use std::collections::btree_set::Iter;
use std::collections::{BTreeSet, HashMap};

const SKIP_LIST_MAX_LEVEL: u8 = 31;
const SKIP_LIST_PROB: f64 = 0.5;

struct ListNode {
    id: u64,
    level: u8,
    pub next: HashMap<u8, u64>,
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

    fn get_values(&self) -> Iter<String> {
        self.values.iter()
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
    list: u64,
    nodes: HashMap<u64, RefCell<ListNode>>,
    next_node_id: u64,
}

impl Default for SkipList {
    fn default() -> Self {
        Self::new()
    }
}

impl SkipList {
    pub fn new() -> Self {
        let head_node = RefCell::new(ListNode::new(0, SKIP_LIST_MAX_LEVEL, f64::MIN));
        let head_id = head_node.borrow().id;
        let tail_node = RefCell::new(ListNode::new(1, SKIP_LIST_MAX_LEVEL, f64::MAX));
        let tail_id = tail_node.borrow().id;
        for level in 0..=SKIP_LIST_MAX_LEVEL {
            head_node.borrow_mut().set_next(level, &tail_node);
        }
        let nodes = HashMap::from([(head_id, head_node), (tail_id, tail_node)]);
        SkipList {
            max_level: SKIP_LIST_MAX_LEVEL,
            prob: SKIP_LIST_PROB,
            list: 0,
            nodes,
            next_node_id: 2,
        }
    }

    fn should_insert(&self, score: f64, value: &String) -> Option<Vec<(u8, u64)>> {
        let mut level: i16 = SKIP_LIST_MAX_LEVEL as i16;
        let mut node_id = 0;
        let mut prev = Vec::<(u8, u64)>::new();
        while level >= 0 {
            let node = self.nodes.get(&node_id).unwrap();
            let node_score = node.borrow().score;
            let next_node_id = node.borrow().get_next(level as u8).unwrap();
            let next_node = self.nodes.get(&next_node_id).unwrap();
            let next_node_score = next_node.borrow().score;
            println!("{}, {}, {}, {}", score, node_score, next_node_score, value);
            if score == node_score {
                node.borrow_mut().add_value(value.clone());
                return None;
            } else if score >= next_node_score {
                node_id = next_node_id;
            } else {
                prev.push((level as u8, node.borrow().id));
                level -= 1;
            }
        }
        Some(prev)
    }

    fn insert_node(&self, new_node: &RefCell<ListNode>, curr_level: u8, node_id: u64) {
        let node = self.nodes.get(&node_id).unwrap();
        let next_next_id_op = node.borrow().get_next(curr_level);
        if let Some(next_next_id) = next_next_id_op {
            let next_node = self.nodes.get(&next_next_id).unwrap();
            new_node.borrow_mut().set_next(curr_level, next_node);
        }
        new_node.borrow_mut().set_level(curr_level);
        node.borrow_mut().set_next(curr_level, new_node);
    }

    pub fn insert(&mut self, score: f64, value: String) {
        let prev_op = self.should_insert(score, &value);
        if prev_op.is_none() {
            return;
        }
        let mut prev = prev_op.unwrap();

        let new_node_id = self.next_node_id;
        let new_node = RefCell::new(ListNode::new(new_node_id, 0, score));
        self.nodes.insert(new_node_id, new_node);
        self.next_node_id += 1;

        let new_node = self.nodes.get(&new_node_id).unwrap();
        new_node.borrow_mut().add_value(value);
        let (curr_level, node_id) = prev.pop().unwrap();
        assert_eq!(curr_level, 0);
        self.insert_node(new_node, curr_level, node_id);

        let mut rng = rand::thread_rng();
        while rng.gen::<f64>() > self.prob {
            if let Some((curr_level, node_id)) = prev.pop() {
                self.insert_node(new_node, curr_level, node_id);
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    mod test_listnode {
        use crate::data_store_helper::sorted_set::ListNode;
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
                n0.borrow().get_values().collect::<Vec<_>>(),
                vec![
                    &"a".to_string(),
                    &"aa".to_string(),
                    &"b".to_string(),
                    &"c".to_string()
                ]
            );
        }
    }

    mod test_skiplist {
        use crate::data_store_helper::sorted_set::SkipList;

        #[test]
        fn should_insert_node() {
            let mut list = SkipList::new();
            list.insert(1.0, "foo".to_string());
            list.insert(3.0, "bar".to_string());
            list.insert(2.0, "baz".to_string());
            list.insert(1.0, "foobar".to_string());
            let mut nodes = list.nodes.values().collect::<Vec<_>>();
            nodes.sort_by(|a, b| a.borrow().id.cmp(&b.borrow().id));
            let expected = vec![
                (f64::MIN, vec![]),
                (f64::MAX, vec![]),
                (1.0, vec!["foo", "foobar"]),
                (3.0, vec!["bar"]),
                (2.0, vec!["baz"]),
            ];
            for i in 0..5 {
                assert_eq!(nodes[i].borrow().score, expected[i].0);
                assert_eq!(
                    nodes[i].borrow().get_values().collect::<Vec<_>>(),
                    expected[i].1
                );
            }
        }
    }
}
