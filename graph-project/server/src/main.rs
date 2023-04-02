use std::{cell::RefCell, collections::HashMap, sync::Arc};

use dotenv::dotenv;
use map::dijkstra::WeightedNode;
use rocket::{http::Status, launch, routes, serde::json::Json};

#[macro_use]
extern crate rocket;

#[get("/", format = "json")]
async fn get_map() -> Result<Json<map::Map>, Status> {
    match map::Map::new().await {
        Ok(map) => Ok(Json(map)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/<id>", format = "json")]
async fn get_shortest_path_for_id(
    id: i64,
) -> Result<Json<HashMap<i64, Arc<RefCell<WeightedNode>>>>, Status> {
    match map::Map::new().await {
        Ok(map) => Ok(Json(map.shortest_path(&id))),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![get_map, get_shortest_path_for_id])
}
