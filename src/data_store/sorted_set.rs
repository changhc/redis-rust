use rand::Rng;
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};

const SKIP_LIST_MAX_LEVEL: u8 = 32;
const SKIP_LIST_PROB: f64 = 0.5;

struct ListNode {
    id: u64,
    level: u8,
    next: HashMap<u8, u64>,
    span: HashMap<u8, u64>,
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
            span: (0..=level).map(|v| (v, 0)).collect::<HashMap<_, _>>(),
            values: BTreeSet::new(),
        }
    }

    pub fn set_level(&mut self, level: u8) {
        self.level = level;
    }

    pub fn add_value(&mut self, value: String) -> bool {
        self.values.insert(value)
    }

    pub fn set_next(&mut self, level: u8, node: &RefCell<ListNode>) {
        self.next.insert(level, node.borrow().id);
    }

    pub fn get_next(&self, level: u8) -> Option<u64> {
        self.next.get(&level).cloned()
    }

    pub fn set_span(&mut self, level: u8, value: u64) {
        self.span.insert(level, value);
    }

    pub fn get_span(&self, level: u8) -> Option<u64> {
        self.span.get(&level).cloned()
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
        let head_node = RefCell::new(ListNode::new(0, max_level, -f64::INFINITY));
        let head_id = head_node.borrow().id;
        let tail_node = RefCell::new(ListNode::new(1, max_level, f64::INFINITY));
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

    fn check_if_node_exists(&self, score: f64) -> (bool, Vec<(u8, u64)>) {
        let mut level: i16 = self.max_level as i16;
        let mut current_node_id = self.head_id;
        // List of the immediate previous node per level. (level, node_id)
        let mut previous_nodes = Vec::<(u8, u64)>::new();
        let mut node_exists = false;
        while level >= 0 {
            let current_node = self.nodes.get(&current_node_id).unwrap();
            let current_node_score = current_node.borrow().score;
            if score == current_node_score {
                node_exists = true;
                while level >= 0 {
                    previous_nodes.push((level as u8, current_node.borrow().id));
                    level -= 1;
                }
                break;
            }

            // It's fine to unwrap. If score is at the end of the list (+inf), this method returns
            // at the previous line. There is no value greater than +inf, so the next node of the
            // end of the list should never be accessed.
            let next_node_id = current_node.borrow().get_next(level as u8).unwrap();
            let next_node = self.nodes.get(&next_node_id).unwrap();
            let next_node_score = next_node.borrow().score;
            if score >= next_node_score {
                current_node_id = next_node_id;
            } else {
                previous_nodes.push((level as u8, current_node.borrow().id));
                level -= 1;
            }
        }
        (node_exists, previous_nodes)
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
        new_node.borrow_mut().set_span(current_level, 0);
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
        let (node_exists, mut previous_nodes) = self.check_if_node_exists(score);
        if node_exists {
            let (_, current_node_id) = previous_nodes.last().unwrap();
            let current_node = self.nodes.get(current_node_id).unwrap();
            assert!(current_node.borrow_mut().add_value(value));
        } else {
            let new_node_id = self.create_new_node(score, &value);
            let new_node = self.nodes.get(&new_node_id).unwrap();
            let (current_level, current_node_id) = previous_nodes.pop().unwrap();
            assert_eq!(current_level, 0);
            let current_node = self.nodes.get(&current_node_id).unwrap();
            self.insert_node_at_level(current_node, new_node, current_level.to_owned());

            let mut rng = rand::thread_rng();
            while !previous_nodes.is_empty() && rng.gen::<f64>() >= self.prob {
                let (current_level, current_node_id) = previous_nodes.pop().unwrap();
                let current_node = self.nodes.get(&current_node_id).unwrap();
                self.insert_node_at_level(current_node, new_node, current_level.to_owned());
            }

            // Push the new node into previous_nodes for span adjustment. The popped previous nodes
            // at each level are replaced by the new node as the new node is the one whose span
            // should get incremented
            let mut level = self.max_level;
            if let Some((current_level, _)) = previous_nodes.last() {
                level = current_level - 1;
            }
            for i in (0..=level).rev() {
                previous_nodes.push((i, new_node_id));
            }
        }

        // Increment span at each level
        for (current_level, current_node_id) in previous_nodes.iter().cloned() {
            let current_node = self.nodes.get(&current_node_id).unwrap();
            let current_span = current_node.borrow().get_span(current_level).unwrap();
            current_node
                .borrow_mut()
                .set_span(current_level, current_span + 1);
        }
    }

    pub fn get_values_by_score(&self, start_score: f64, stop_score: f64) -> Vec<String> {
        let mut result = Vec::new();
        let mut level: i16 = self.max_level as i16;
        let mut current_node_id = self.head_id;
        while level >= 0 {
            let current_node = self.nodes.get(&current_node_id).unwrap();
            let current_node_score = current_node.borrow().score;
            if start_score == current_node_score {
                break;
            }
            let next_node_id = current_node.borrow().get_next(level as u8).unwrap();
            let next_node = self.nodes.get(&next_node_id).unwrap();
            let next_node_score = next_node.borrow().score;
            if start_score >= next_node_score {
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
            if let Some(next_node_id) = current_node.borrow().get_next(0) {
                current_node = self.nodes.get(&next_node_id).unwrap();
                current_node_score = current_node.borrow().score;
            } else {
                // Reached the end of the list (+inf); cannot proceed further
                break;
            }
        }
        result
    }

    pub fn get_values_by_rank(&self, start_rank: u64, stop_rank: u64) -> Vec<String> {
        // input rank numbers are 0-based
        let start_rank = start_rank + 1;
        let stop_rank = stop_rank + 1;
        let mut result = Vec::new();
        let mut level: i16 = self.max_level as i16;
        let mut current_node_id = self.head_id;
        let mut num_seen_values = 0;
        while level >= 0 {
            let current_node = self.nodes.get(&current_node_id).unwrap();
            let current_node_span = current_node.borrow().get_span(level as u8).unwrap();
            if start_rank <= num_seen_values + current_node_span {
                break;
            }
            let next_node_id = current_node.borrow().get_next(level as u8).unwrap();
            let next_node = self.nodes.get(&next_node_id).unwrap();
            let next_node_span = next_node.borrow().get_span(level as u8).unwrap();
            if start_rank >= num_seen_values + current_node_span + next_node_span {
                current_node_id = next_node_id;
            } else {
                level -= 1;
            }
        }
        let mut current_node = self.nodes.get(&current_node_id).unwrap();
        while num_seen_values < stop_rank {
            for v in current_node.borrow().values.iter() {
                num_seen_values += 1;
                if num_seen_values >= start_rank {
                    result.push(v.to_owned());
                }
                if num_seen_values == stop_rank {
                    break;
                }
            }
            if num_seen_values == stop_rank {
                break;
            }
            if let Some(next_node_id) = current_node.borrow().get_next(0) {
                current_node = self.nodes.get(&next_node_id).unwrap();
            } else {
                // Reached the end of the list (+inf); cannot proceed further
                break;
            }
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
            list.insert(f64::INFINITY, "inf".to_string());
            let mut nodes = list.nodes.values().collect::<Vec<_>>();
            nodes.sort_by(|a, b| a.borrow().id.cmp(&b.borrow().id));
            assert_eq!(nodes.len(), 5);
            let expected = [
                (-f64::INFINITY, vec![]),
                (f64::INFINITY, vec!["inf"]),
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
        fn should_accumulate_span() {
            let mut list = SkipList::new(2);
            // set prob to 1 so that only the head node has level > 0
            list.prob = 1.0;
            for i in 0..5 {
                list.insert(i as f64, i.to_string());
            }
            let n0 = list.nodes.get(&2).unwrap();
            assert_eq!(n0.borrow().get_span(0).unwrap(), 1);

            let head = list.nodes.get(&0).unwrap();
            assert_eq!(head.borrow().get_span(2).unwrap(), 5);
            assert_eq!(head.borrow().get_span(1).unwrap(), 5);
            assert_eq!(head.borrow().get_span(0).unwrap(), 0);

            list.insert(-f64::INFINITY, "inf".to_string());
            let head = list.nodes.get(&0).unwrap();
            assert_eq!(head.borrow().get_span(2).unwrap(), 6);
            assert_eq!(head.borrow().get_span(1).unwrap(), 6);
            assert_eq!(head.borrow().get_span(0).unwrap(), 1);
        }

        #[test]
        fn insert_node_should_return_none_when_scores_exist() {
            let mut list = SkipList::new(2);
            // set prob to -1 so that nodes are always created in order to remove randomness
            list.prob = -1.0;

            let res = list.check_if_node_exists(1.0);
            assert_eq!(res, (false, vec![(2, 0), (1, 0), (0, 0)]));
            list.insert(1.0, "a".to_string());
            let res = list.check_if_node_exists(1.0);
            assert_eq!(res, (true, vec![(2, 2), (1, 2), (0, 2)]));
            let res = list.check_if_node_exists(3.0);
            assert_eq!(res, (false, vec![(2, 2), (1, 2), (0, 2)]));
            let res = list.check_if_node_exists(2.0);
            assert_eq!(res, (false, vec![(2, 2), (1, 2), (0, 2)]));
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
            assert_eq!(n2.borrow().get_next(0).unwrap(), 3);
        }

        #[test]
        fn should_get_values_by_score() {
            let mut list = SkipList::new(2);
            // Remove randomness
            list.prob = 1.0;
            let input = [
                (1.0, "a"),
                (3.0, "b"),
                (2.0, "c"),
                (1.0, "d"),
                (3.9, "e"),
                (f64::INFINITY, "f"),
                (-f64::INFINITY, "g"),
            ];
            for (score, value) in input {
                list.insert(score, value.to_string());
            }
            let values = list.get_values_by_score(-1.0, 4.0);
            assert_eq!(values, ["a", "d", "c", "b", "e"]);
            let values = list.get_values_by_score(1.5, 4.0);
            assert_eq!(values, ["c", "b", "e"]);
            let values = list.get_values_by_score(1.5, 3.5);
            assert_eq!(values, ["c", "b"]);
            let values = list.get_values_by_score(1.5, 1.9);
            assert!(values.is_empty());
            let values = list.get_values_by_score(2.0, 1.9);
            assert!(values.is_empty());
            let values = list.get_values_by_score(4.0, f64::INFINITY);
            assert_eq!(values, ["f"]);
            let values = list.get_values_by_score(-f64::INFINITY, 1.0000001);
            assert_eq!(values, ["g", "a", "d"]);
        }

        #[test]
        fn should_get_values_by_rank() {
            let mut list = SkipList::new(2);
            // Remove randomness
            list.prob = 1.0;
            let input = [
                (1.0, "a"),
                (3.0, "b"),
                (2.0, "c"),
                (1.0, "d"),
                (3.9, "e"),
                (f64::INFINITY, "f"),
            ];
            for (score, value) in input {
                list.insert(score, value.to_string());
            }
            let values = list.get_values_by_rank(1, 4);
            assert_eq!(values, ["d", "c", "b", "e"]);
            let values = list.get_values_by_rank(3, 8);
            assert_eq!(values, ["b", "e", "f"]);
            let values = list.get_values_by_rank(0, 1);
            assert_eq!(values, ["a", "d"]);
            let values = list.get_values_by_rank(0, 0);
            assert_eq!(values, ["a"]);
            let values = list.get_values_by_rank(2, 0);
            assert!(values.is_empty());

            // Test if code behaves when the start node holds values
            list.insert(-f64::INFINITY, "g".to_string());
            let values = list.get_values_by_rank(0, 1);
            assert_eq!(values, ["g", "a"]);
        }
    }
}
