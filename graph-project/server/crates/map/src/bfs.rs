use std::{
    collections::{HashSet, VecDeque, HashMap},
    sync::Arc,
};

use crate::{Map, Node};

impl Map {
    /// Could be used for find the path used aswell
    pub fn bfs(&self, from: &i64, road_name: &str) -> Option<i64> {
        let Some(root) = self.nodes.get(from) else {
            return None;
        };
        let mut queue = VecDeque::<Arc<Node>>::new();
        let mut visited = HashMap::<i64, Option<i64>>::new();
        visited.insert(root.id, None);
        queue.push_front(Arc::clone(root));
        while let Some(node) = queue.pop_front() {
            if node.road_name == road_name {
                print_path(node.id, visited);
                return Some(node.id);
            }
            let edges = self.relations.get(&node.id).unwrap();
            for edge in edges {
                let rel_node = if edge.node_one.id != node.id {
                    &edge.node_one
                } else {
                    &edge.node_two
                };
                if !visited.contains_key(&rel_node.id) {
                    queue.push_back(Arc::clone(rel_node));
                    visited.insert(rel_node.id, Some(node.id));
                }
            }
        }
        None
    }
}

fn print_path(found_id: i64, visited: HashMap<i64, Option<i64>>) {
    let mut curr = Some(found_id);
    while curr.is_some() {
        println!("-> {}", curr.unwrap());
        curr = *visited.get(&curr.unwrap()).unwrap();
    }
}
