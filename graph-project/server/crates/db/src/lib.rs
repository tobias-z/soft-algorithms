use model::{RoadPart, RoadPartRelation};
use sqlx::{Pool, Postgres};

pub mod connection;
pub mod model;

pub struct RoadService {
    pool: Pool<Postgres>,
}

impl RoadService {
    pub async fn new() -> sqlx::Result<Self> {
        let pool = connection::connect(5).await?;
        Ok(Self { pool })
    }

    pub async fn get_road_parts(&self) -> Result<Vec<RoadPart>, sqlx::Error> {
        sqlx::query_as(
            "
SELECT rp.id, road.road_name, rp.road_id FROM road_part as rp
INNER JOIN road
ON rp.road_id = road.id;
",
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_relations_of_part(
        &self,
        part_id: i64,
    ) -> Result<Vec<RoadPartRelation>, sqlx::Error> {
        sqlx::query_as(
            "
SELECT part_one, part_two, weight FROM road_part_relation
WHERE part_one = $1 OR part_two = $1;
",
        )
        .bind(part_id)
        .fetch_all(&self.pool)
        .await
    }
}
