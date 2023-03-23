CREATE TABLE road (
    id BIGSERIAL PRIMARY KEY,
    road_name VARCHAR(100)
);

CREATE TABLE road_part (
    id BIGSERIAL PRIMARY KEY,
    road_id BIGSERIAL NOT NULL,
    FOREIGN KEY (road_id) REFERENCES road (id)
);

CREATE TABLE road_pard_relation (
    part_one BIGSERIAL NOT NULL,
    part_two BIGSERIAL NOT NULL,
    weight INT NOT NULL,
    FOREIGN KEY (part_one) REFERENCES road_part (id),
    FOREIGN KEY (part_two) REFERENCES road_part (id)
);
