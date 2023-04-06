use std::{cell::RefCell, collections::HashMap, sync::Arc};

use dotenv::dotenv;
use map::dijkstra::WeightedNode;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{Header, Status},
    launch, routes,
    serde::json::Json,
    Request, Response,
};

#[macro_use]
extern crate rocket;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

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
        Ok(map) => match map.shortest_path(&id) {
            Some(paths) => Ok(Json(paths)),
            None => Err(Status::NotFound),
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().attach(CORS).mount("/", routes![get_map, get_shortest_path_for_id])
}
