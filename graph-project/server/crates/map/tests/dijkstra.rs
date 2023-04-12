use common::with_correct_env;
use map::Map;

pub mod common;

#[actix_rt::test]
async fn shortest_path_for_same_node_should_be_zero() {
    with_correct_env(async {
        let map = Map::new().await.expect("Map was not found");
        let nodes = map.shortest_path(&1).unwrap();
        assert_eq!(nodes.get(&1).unwrap().borrow().node.id, 1);
        assert_eq!(nodes.get(&1).unwrap().borrow().weight, 0);
    })
    .await;
}

#[actix_rt::test]
async fn closely_linked() {
    with_correct_env(async {
        let map = Map::new().await.expect("Map was not found");
        let nodes = map.shortest_path(&1).unwrap();
        assert_eq!(nodes.get(&2).unwrap().borrow().node.id, 2);
        // 0 + 1
        assert_eq!(nodes.get(&2).unwrap().borrow().weight, 1);
    })
    .await;
}

#[actix_rt::test]
async fn weights_should_be_added_together() {
    with_correct_env(async {
        let map = Map::new().await.expect("Map was not found");
        let nodes = map.shortest_path(&1).unwrap();
        assert_eq!(nodes.get(&3).unwrap().borrow().node.id, 3);
        // // 0 + 1 + 2
        // 3.prev_node.weight == 1
        assert_eq!(nodes.get(&3).unwrap().borrow().weight, 3);
    })
    .await;
}
