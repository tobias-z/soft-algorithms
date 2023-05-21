use common::with_correct_env;
use map::Map;

pub mod common;

#[actix_rt::test]
async fn can_return_a_map() {
    with_correct_env(async {
        let map = Map::new().await;
        assert!(map.is_ok());
    })
    .await;
}

#[actix_rt::test]
#[should_panic]
async fn will_fail_if_no_env() {
    let map = Map::new().await;
    assert!(map.is_err());
}

#[actix_rt::test]
async fn map_has_nodes() {
    with_correct_env(async {
        let map = Map::new().await.expect("Map was not found");
        assert!(!map.nodes.is_empty());
    })
    .await;
}

#[actix_rt::test]
async fn map_has_relations() {
    with_correct_env(async {
        let map = Map::new().await.expect("Map was not found");
        assert!(!map.nodes.is_empty());
    })
    .await;
}

#[actix_rt::test]
async fn can_find_node_with_dfs() {
    with_correct_env(async {
        let map = Map::new().await.expect("Map was not found");
        let val = map.find_roadpart_id("skovvej");
        assert!(val.is_some())
    })
    .await;
}

#[actix_rt::test]
async fn can_find_node_with_bfs() {
    with_correct_env(async {
        let map = Map::new().await.expect("Map was not found");
        let val = map.bfs(&1, "skovvej");
        println!("------------------------");
        let val = map.bfs(&27, "skovvej");
        assert!(val.is_some())
    })
    .await;
}
