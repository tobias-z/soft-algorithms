use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    sync::Arc,
};

use serde::{Deserialize, Serialize};

use crate::{Map, Node};

const INFINITY: u32 = 999999;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WeightedNode {
    pub node: Arc<Node>,
    pub prev_node: Option<Arc<RefCell<WeightedNode>>>,
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
    // TODO: should be looking for a specific node
    pub fn shortest_path(
        &self,
        from_node: &i64,
    ) -> Option<HashMap<i64, Arc<RefCell<WeightedNode>>>> {
        if !self.nodes.contains_key(from_node) {
            return None;
        }

        let mut unvisited: BinaryHeap<Arc<RefCell<WeightedNode>>> = BinaryHeap::new();
        let all_nodes: HashMap<i64, Arc<RefCell<WeightedNode>>> = self
            .nodes
            .iter()
            .map(|(id, node)| {
                (
                    *id,
                    Arc::new(RefCell::new(WeightedNode {
                        node: Arc::clone(node),
                        prev_node: None,
                        weight: if &node.id == from_node { 0 } else { INFINITY },
                    })),
                )
            })
            .collect();
        unvisited.push(Arc::clone(all_nodes.get(from_node).unwrap()));
        let mut visited = HashMap::new();

        while let Some(next) = unvisited.pop() {
            let relations = self.relations.get(&next.borrow().node.id).unwrap();
            for relation in relations {
                let related_id = if relation.node_one.id == next.borrow().node.id {
                    &relation.node_two.id
                } else {
                    &relation.node_one.id
                };
                if !visited.contains_key(related_id) {
                    let mut related_node = all_nodes.get(related_id).unwrap().borrow_mut();
                    let next_weight = next.borrow().weight + relation.weight;
                    if next_weight < related_node.weight {
                        related_node.weight = next_weight;
                        related_node.prev_node = Some(Arc::clone(&next));
                    }
                    drop(related_node);
                    unvisited.push(Arc::clone(all_nodes.get(related_id).unwrap()));
                }
            }
            visited.insert(next.borrow().node.id, Arc::clone(&next));
        }
        Some(visited)
    }
}
