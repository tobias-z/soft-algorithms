use std::{collections::HashSet, sync::Arc};

use crate::{Map, Node};

impl Map {
    pub fn find_roadpart_id(&self, road_name: &str) -> Option<i64> {
        match self.nodes.values().next() {
            Some(node) => self.dfs(road_name, node, &mut HashSet::new()),
            None => None,
        }
    }

    fn dfs(&self, road_name: &str, node: &Arc<Node>, visited: &mut HashSet<i64>) -> Option<i64> {
        if !visited.insert(node.id) {
            return None;
        }
        if node.road_name == road_name {
            return Some(node.id);
        }
        let relations = self.relations.get(&node.id).unwrap();
        for relation in relations {
            if relation.node_one.id != node.id {
                if let Some(id) = self.dfs(road_name, &relation.node_one, visited) {
                    return Some(id);
                }
            }
            if let Some(id) = self.dfs(road_name, &relation.node_two, visited) {
                return Some(id);
            }
        }
        None
    }
}
