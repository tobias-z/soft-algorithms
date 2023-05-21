pub mod dfs;
pub mod dijkstra;
pub mod bfs;

use std::{collections::HashMap, sync::Arc};

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
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: i64,
    pub road_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Relation {
    pub node_one: Arc<Node>,
    pub node_two: Arc<Node>,
    pub weight: u32,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
}
