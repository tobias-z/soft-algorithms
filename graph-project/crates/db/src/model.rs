pub struct RoadPart {
    pub id: i64,
    pub road_name: String,
    pub road_id: i64,
}

pub struct RoadPartRelation {
    pub part_one: i64,
    pub part_two: i64,
    pub weight: i64,
}
