use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use anyhow::anyhow;
use db::{connection::PostgresConnection, model::RoadPart};

impl From<RoadPart> for Node {
    fn from(road_part: RoadPart) -> Self {
        Self {
            id: road_part.id,
            road_name: road_part.road_name,
        }
    }
}

pub struct Node {
    pub id: i64,
    pub road_name: String,
}

pub struct Relation {
    pub node_one: Rc<Node>,
    pub node_two: Rc<Node>,
    pub weight: u32,
}

#[derive(Default)]
pub struct Map {
    /// gets the nodes from the database and inserts them into a hashmap.
    /// we use a hashmap here for easy id lookups when creating the relations
    nodes: HashMap<i64, Rc<Node>>,

    /// relations are also kept as a hashmap to make finding a specific node in question very fast
    relations: HashMap<i64, Vec<Relation>>,
}

impl Map {
    pub async fn new() -> anyhow::Result<Self> {
        let mut map = Self::default();
        map.generate_map().await?;
        Ok(map)
    }

    async fn generate_map(&mut self) -> anyhow::Result<()> {
        let road_service = db::RoadService::new(Box::new(PostgresConnection::new(5))).await;
        self.fill_nodes(&road_service).await?;
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
                    node_one: Rc::clone(self.nodes.get(&part_relation.part_one).unwrap()),
                    node_two: Rc::clone(self.nodes.get(&part_relation.part_two).unwrap()),
                    weight: part_relation.weight as u32,
                });
            }
            self.relations.insert(node.id, node_relations);
        }
        Ok(())
    }

    async fn fill_nodes(&mut self, road_service: &db::RoadService) -> anyhow::Result<()> {
        let road_parts = road_service.get_road_parts().await?;
        for road_part in road_parts {
            self.nodes.insert(road_part.id, Rc::new(road_part.into()));
        }
        Ok(())
    }

    pub fn find_roadpart_id(&self, road_name: &str) -> Option<i64> {
        match self.nodes.values().next() {
            Some(node) => self.dfs(road_name, node, &mut HashSet::new()),
            None => None,
        }
    }

    fn dfs(&self, road_name: &str, node: &Rc<Node>, visited: &mut HashSet<i64>) -> Option<i64> {
        if !visited.insert(node.id) {
            return None;
        }
        if node.road_name == road_name {
            return Some(node.id);
        }
        let relations = self.relations.get(&node.id).unwrap();
        for relation in relations {
            if relation.node_one.id != node.id {
                if let Some(val) = self.dfs(road_name, &relation.node_one, visited) {
                    return Some(val);
                }
            }
            if let Some(val) = self.dfs(road_name, &relation.node_two, visited) {
                return Some(val);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    async fn with_correct_env<F>(closure: F)
    where
        F: std::future::Future<Output = ()> + std::future::IntoFuture,
    {
        temp_env::async_with_vars(
            [
                ("POSTGRES_USER", Some("postgres")),
                ("POSTGRES_PASSWORD", Some("postgres")),
                ("POSTGRES_HOST", Some("localhost")),
                ("POSTGRES_DATABASE", Some("postgres")),
            ],
            closure,
        )
        .await;
    }

    #[actix_rt::test]
    async fn can_return_a_map() {
        async fn test() {
            let map = Map::new().await;
            assert!(map.is_ok());
        }
        with_correct_env(test()).await;
    }

    #[actix_rt::test]
    #[should_panic]
    async fn will_fail_if_no_env() {
        let map = Map::new().await;
        assert!(map.is_err());
    }

    #[actix_rt::test]
    async fn map_has_nodes() {
        async fn test() {
            let map = Map::new().await.expect("Map was not found");
            assert!(!map.nodes.is_empty());
        }
        with_correct_env(test()).await;
    }

    #[actix_rt::test]
    async fn map_has_relations() {
        async fn test() {
            let map = Map::new().await.expect("Map was not found");
            assert!(!map.nodes.is_empty());
        }
        with_correct_env(test()).await;
    }

    #[actix_rt::test]
    async fn can_find_node_with_dfs() {
        async fn test() {
            let map = Map::new().await.expect("Map was not found");
            let val = map.find_roadpart_id("skovvej");
            println!("{}", val.unwrap());
        }
        with_correct_env(test()).await;
    }
}
