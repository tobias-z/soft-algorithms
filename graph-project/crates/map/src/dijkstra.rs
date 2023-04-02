use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    rc::Rc,
};

use crate::{Map, Node};

#[derive(Clone)]
pub struct WeightedNode {
    pub node: Rc<Node>,
    pub prev_node: Option<Rc<RefCell<WeightedNode>>>,
    pub weight: u32,
}

impl Eq for WeightedNode {}

impl PartialEq for WeightedNode {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl PartialOrd for WeightedNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight
            .partial_cmp(&other.weight)
            .map(|ordering| ordering.reverse())
    }
}

impl Ord for WeightedNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight).reverse()
    }
}

impl Map {
    pub fn shortest_path(&self, from_node: &i64) -> HashMap<i64, WeightedNode> {
        let mut unvisited: BinaryHeap<Rc<RefCell<WeightedNode>>> = self
            .nodes
            .values()
            .map(|node| {
                Rc::new(RefCell::new(WeightedNode {
                    node: Rc::clone(node),
                    prev_node: None,
                    weight: if &node.id == from_node { 0 } else { 999999 },
                }))
            })
            .collect();
        let all_nodes: HashMap<i64, Rc<RefCell<WeightedNode>>> = unvisited
            .iter()
            .map(|node| (node.borrow().node.id, Rc::clone(node)))
            .collect();
        let mut visited = HashMap::new();

        while let Some(next) = unvisited.pop() {
            let relations = self.relations.get(&next.borrow().node.id).unwrap();
            for relation in relations {
                let related_id = if relation.node_one.id == next.borrow().node.id {
                    &relation.node_two.id
                } else {
                    &relation.node_one.id
                };
                if visited.contains_key(related_id) {
                    continue;
                }
                let mut related_node = all_nodes.get(related_id).unwrap().borrow_mut();
                let next_weight = next.borrow().weight + relation.weight;
                if next_weight < related_node.weight {
                    related_node.weight = next_weight;
                    related_node.prev_node = Some(Rc::clone(&next));
                }
            }
            visited.insert(next.borrow().node.id, Rc::clone(&next));
        }
        visited
            .into_iter()
            .map(|(id, node)| (id, node.borrow().clone()))
            .collect()
    }
}
