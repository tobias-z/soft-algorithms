pub mod dijkstra;

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use anyhow::{anyhow, Result};
use db::model::RoadPart;
use serde::{Deserialize, Serialize};

impl From<RoadPart> for Node {
    fn from(road_part: RoadPart) -> Self {
        Self {
            id: road_part.id,
            road_name: road_part.road_name,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Node {
    pub id: i64,
    pub road_name: String,
}

unsafe impl Sync for Node {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relation {
    pub node_one: Arc<Node>,
    pub node_two: Arc<Node>,
    pub weight: u32,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Map {
    /// gets the nodes from the database and inserts them into a hashmap.
    /// we use a hashmap here for easy id lookups when creating the relations
    pub nodes: HashMap<i64, Arc<Node>>,

    /// relations are also kept as a hashmap to make finding a specific node in question very fast
    relations: HashMap<i64, Vec<Relation>>,
}

impl Map {
    pub async fn new() -> Result<Self> {
        let mut map = Self::default();
        map.generate_map().await?;
        Ok(map)
    }

    async fn generate_map(&mut self) -> Result<()> {
        let road_service = db::RoadService::new().await?;
        let road_parts = road_service.get_road_parts().await?;
        for road_part in road_parts {
            self.nodes.insert(road_part.id, Arc::new(road_part.into()));
        }
        for node in self.nodes.values() {
            let part_relations = road_service.get_relations_of_part(node.id).await?;
            let mut node_relations = vec![];
            // fill in all relations with their respective nodes
            for part_relation in part_relations {
                if part_relation.weight.is_negative() {
                    return Err(anyhow!(
                        "Weights should never be negative. Your data is wrong"
                    ));
                }

                node_relations.push(Relation {
                    node_one: Arc::clone(self.nodes.get(&part_relation.part_one).unwrap()),
                    node_two: Arc::clone(self.nodes.get(&part_relation.part_two).unwrap()),
                    weight: part_relation.weight as u32,
                });
            }
            self.relations.insert(node.id, node_relations);
        }
        Ok(())
    }

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
