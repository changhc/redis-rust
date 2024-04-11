use rand::Rng;
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};

const SKIP_LIST_MAX_LEVEL: u8 = 32;
const SKIP_LIST_PROB: f64 = 0.5;

pub struct SortedSet {
    elements: HashMap<String, f64>,
    skip_list: SkipList,
}

impl Default for SortedSet {
    fn default() -> Self {
        Self::new()
    }
}

impl SortedSet {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            skip_list: SkipList::new(SKIP_LIST_MAX_LEVEL),
        }
    }

    pub fn insert(&mut self, score: f64, element: String) -> bool {
        let mut is_new_element = true;
        if let Some(current_score) = self.elements.get(&element) {
            self.skip_list.remove(*current_score, &element);
            is_new_element = false;
        }
        self.elements.insert(element.clone(), score);
        self.skip_list.insert(score, element);
        is_new_element
    }

    pub fn remove(&mut self, element: &str) -> bool {
        if let Some(score) = self.elements.get(element) {
            self.skip_list.remove(*score, element);
            self.elements.remove(element);
            return true;
        }
        false
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn get(&self, element: &str) -> Option<f64> {
        self.elements.get(element).cloned()
    }

    pub fn get_values_by_rank(&self, start: u64, stop: u64) -> Vec<String> {
        self.skip_list.get_values_by_rank(start, stop)
    }

    pub fn get_rank(&self, element: &str) -> Option<u64> {
        self.elements
            .get(element)
            .map(|score| self.skip_list.get_rank(score, element))
    }
}

struct ListNode {
    id: u64,
    level: u8,
    next: Vec<Option<u64>>,
    prev: Vec<Option<u64>>,
    span: Vec<u64>,
    score: f64,
    values: BTreeSet<String>,
}

impl ListNode {
    pub fn new(id: u64, level: u8, score: f64) -> Self {
        ListNode {
            id,
            level,
            score,
            next: vec![None; level as usize + 1],
            prev: vec![None; level as usize + 1],
            span: vec![0u64; level as usize + 1],
            values: BTreeSet::new(),
        }
    }

    pub fn set_level(&mut self, level: u8) {
        self.level = level;
    }

    pub fn add_value(&mut self, value: String) -> bool {
        self.values.insert(value)
    }

    pub fn remove_value(&mut self, value: &str) -> bool {
        self.values.remove(value)
    }

    pub fn set_next(&mut self, level: u8, node: &RefCell<ListNode>) {
        self.next[level as usize] = Some(node.borrow().id);
    }

    pub fn get_next(&self, level: u8) -> Option<u64> {
        self.next[level as usize]
    }

    pub fn set_prev(&mut self, level: u8, node: &RefCell<ListNode>) {
        self.prev[level as usize] = Some(node.borrow().id);
    }

    pub fn get_prev(&self, level: u8) -> Option<u64> {
        self.prev[level as usize]
    }

    pub fn set_span(&mut self, level: u8, value: u64) {
        self.span[level as usize] = value;
    }

    pub fn get_span(&self, level: u8) -> u64 {
        self.span[level as usize]
    }
}

struct SkipList {
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
            tail_node.borrow_mut().set_prev(level, &head_node);
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
            next_node.borrow_mut().set_prev(current_level, new_node);
        }
        new_node.borrow_mut().set_level(current_level);
        current_node.borrow_mut().set_next(current_level, new_node);
        new_node.borrow_mut().set_prev(current_level, current_node);
    }

    fn create_new_node(&mut self, level: u8, score: f64, value: &str) -> u64 {
        let new_node_id = self.next_node_id;
        let new_node = RefCell::new(ListNode::new(new_node_id, level, score));
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
            let mut level = 0;
            let mut rng = rand::thread_rng();
            while level < self.max_level && rng.gen::<f64>() >= self.prob {
                level += 1;
            }
            let new_node_id = self.create_new_node(level, score, &value);
            let new_node = self.nodes.get(&new_node_id).unwrap();
            let len = previous_nodes.len();
            for i in 0..=level {
                let (current_level, current_node_id) = previous_nodes[len - 1 - i as usize];
                let current_node = self.nodes.get(&current_node_id).unwrap();
                self.insert_node_at_level(current_node, new_node, current_level.to_owned());
                // The previous node id at each level are replaced by the new node as the new node
                // is the one whose span should get incremented
                previous_nodes[len - 1 - i as usize] = (current_level, new_node_id);
            }
        }

        // Increment span at each level
        for (current_level, current_node_id) in previous_nodes.iter().cloned() {
            let current_node = self.nodes.get(&current_node_id).unwrap();
            let current_span = current_node.borrow().get_span(current_level);
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
            let current_node_span = current_node.borrow().get_span(level as u8);
            if start_rank <= num_seen_values + current_node_span {
                break;
            }
            let next_node_id = current_node.borrow().get_next(level as u8).unwrap();
            let next_node = self.nodes.get(&next_node_id).unwrap();
            let next_node_span = next_node.borrow().get_span(level as u8);
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

    pub fn get_rank(&self, score: &f64, value: &str) -> u64 {
        let mut level: i16 = self.max_level as i16;
        let mut current_node_id = self.head_id;
        let mut num_seen_values = 0;
        while level >= 0 {
            let current_node = self.nodes.get(&current_node_id).unwrap();
            let current_node_score = current_node.borrow().score;
            if score == &current_node_score {
                break;
            }
            let next_node_id = current_node.borrow().get_next(level as u8).unwrap();
            let next_node = self.nodes.get(&next_node_id).unwrap();
            let next_node_score = next_node.borrow().score;
            if score >= &next_node_score {
                current_node_id = next_node_id;
                num_seen_values += current_node.borrow().get_span(level as u8);
            } else {
                level -= 1;
            }
        }
        let current_node = self.nodes.get(&current_node_id).unwrap();
        let current_node_score = current_node.borrow().score;
        assert!(score == &current_node_score);
        for v in current_node.borrow().values.iter() {
            num_seen_values += 1;
            if v == value {
                break;
            }
        }
        num_seen_values - 1
    }

    pub fn remove(&mut self, score: f64, value: &str) {
        let (node_exists, previous_nodes) = self.check_if_node_exists(score);
        if !node_exists {
            return;
        }
        let (current_level, current_node_id) = previous_nodes.last().unwrap();
        assert_eq!(current_level, &0);
        let current_node = self.nodes.get(current_node_id).unwrap();
        if !current_node.borrow_mut().remove_value(value) {
            return;
        }
        for (current_level, current_node_id) in previous_nodes.iter().cloned() {
            let current_node = self.nodes.get(&current_node_id).unwrap();
            let current_span = current_node.borrow().get_span(current_level);
            current_node
                .borrow_mut()
                .set_span(current_level, current_span - 1);
        }

        // Remove the node from which the value is removed if it has no remaining value at all
        let should_remove = current_node_id > &1 && current_node.borrow().values.is_empty();
        if should_remove {
            self.remove_node(current_node_id);
        }
    }

    fn remove_node(&mut self, current_node_id: &u64) {
        let current_node = self.nodes.get(current_node_id).unwrap();
        for i in 0..=current_node.borrow().level {
            let previous_node_id = current_node.borrow().get_prev(i).unwrap();
            let previous_node = self.nodes.get(&previous_node_id).unwrap();
            let next_node_id = current_node.borrow().get_next(i).unwrap();
            let next_node = self.nodes.get(&next_node_id).unwrap();
            previous_node.borrow_mut().set_next(i, next_node);
            next_node.borrow_mut().set_prev(i, previous_node);
        }
        self.nodes.remove(current_node_id);
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
            assert_eq!(n0.borrow().get_span(0), 1);

            let head = list.nodes.get(&0).unwrap();
            assert_eq!(head.borrow().get_span(2), 5);
            assert_eq!(head.borrow().get_span(1), 5);
            assert_eq!(head.borrow().get_span(0), 0);

            list.insert(-f64::INFINITY, "inf".to_string());
            let head = list.nodes.get(&0).unwrap();
            assert_eq!(head.borrow().get_span(2), 6);
            assert_eq!(head.borrow().get_span(1), 6);
            assert_eq!(head.borrow().get_span(0), 1);
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
            let node_id = list.create_new_node(0, 1.0, "foo");
            assert_eq!(node_id, 2);
            assert_eq!(list.next_node_id, 3);
        }

        #[test]
        fn should_insert_node_at_level() {
            let mut list = SkipList::new(2);
            list.prob = 1.0;
            list.create_new_node(0, 1.0, "a");
            list.create_new_node(0, 3.0, "b");
            list.create_new_node(0, 2.0, "c");
            let n0 = list.nodes.get(&2).unwrap();
            let n1 = list.nodes.get(&3).unwrap();
            let n2 = list.nodes.get(&4).unwrap();
            n0.borrow_mut().set_next(0, n1);
            list.insert_node_at_level(n0, n2, 0);
            assert_eq!(n0.borrow().get_next(0).unwrap(), 4);
            assert_eq!(n2.borrow().get_prev(0).unwrap(), 2);
            assert_eq!(n2.borrow().get_next(0).unwrap(), 3);
            assert_eq!(n1.borrow().get_prev(0).unwrap(), 4);
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

        #[test]
        fn should_remove_value() {
            let mut list = SkipList::new(2);
            // set prob to -1 so that nodes are always created in order to remove randomness
            list.prob = 1.0;

            list.insert(1.0, "foo".to_string());
            list.insert(1.0, "bar".to_string());
            list.insert(0.0, "baz".to_string());
            assert_eq!(list.get_values_by_score(1.0, 1.0), ["bar", "foo"]);
            let node = list.nodes.get(&2).unwrap();
            assert_eq!(node.borrow().get_span(0), 2);
            let head = list.nodes.get(&0).unwrap();
            assert_eq!(head.borrow().get_span(0), 0);
            assert_eq!(head.borrow().get_span(1), 3);
            assert_eq!(head.borrow().get_span(2), 3);

            list.remove(1.0, "bar");
            assert_eq!(list.get_values_by_score(1.0, 1.0), ["foo"]);

            let node = list.nodes.get(&2).unwrap();
            assert_eq!(node.borrow().get_span(0), 1);
            let head = list.nodes.get(&0).unwrap();
            assert_eq!(head.borrow().get_span(0), 0);
            assert_eq!(head.borrow().get_span(1), 2);
            assert_eq!(head.borrow().get_span(2), 2);
        }

        #[test]
        fn should_remove_node_from_list_when_node_is_empty() {
            let mut list = SkipList::new(2);
            // Remove randomness
            list.prob = 1.0;

            list.insert(1.0, "foo".to_string());
            assert_eq!(list.get_values_by_score(1.0, 1.0), ["foo"]);

            list.remove(1.0, "foo");
            assert!(list.get_values_by_score(1.0, 1.0).is_empty());

            assert_eq!(list.nodes.len(), 2);
            assert!(list.nodes.get(&2).is_none());
            let head = list.nodes.get(&0).unwrap();
            let tail = list.nodes.get(&1).unwrap();
            assert_eq!(head.borrow().get_next(0).unwrap(), 1);
            assert_eq!(tail.borrow().get_prev(0).unwrap(), 0);
        }
    }
}
