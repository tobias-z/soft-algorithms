DO $$
DECLARE lærkevej road.id%TYPE;
BEGIN
    INSERT INTO road (road_name)
    VALUES ("lærkevej")
    RETURNING id INTO lærkevej;
END$$;

DO $$
DECLARE birkevej road.id%TYPE;
BEGIN
    INSERT INTO road (road_name)
    VALUES ("birkevej")
    RETURNING id INTO birkevej;
END$$;

DO $$
DECLARE vibevej road.id%TYPE;
BEGIN
    INSERT INTO road (road_name)
    VALUES ("vibevej")
    RETURNING id INTO vibevej;
END$$;

DO $$
DECLARE vinkelvej road.id%TYPE;
BEGIN
    INSERT INTO road (road_name)
    VALUES ("vinkelvej")
    RETURNING id INTO vinkelvej;
END$$;

DO $$
DECLARE søndergade road.id%TYPE;
BEGIN
    INSERT INTO road (road_name)
    VALUES ("søndergade")
    RETURNING id INTO søndergade;
END$$;

DO $$
DECLARE bakkevej road.id%TYPE;
BEGIN
    INSERT INTO road (road_name)
    VALUES ("bakkevej")
    RETURNING id INTO bakkevej;
END$$;

DO $$
DECLARE nørregade road.id%TYPE;
BEGIN
    INSERT INTO road (road_name)
    VALUES ("nørregade")
    RETURNING id INTO nørregade;
END$$;

DO $$
DECLARE drosselvej road.id%TYPE;
BEGIN
    INSERT INTO road (road_name)
    VALUES ("drosselvej")
    RETURNING id INTO drosselvej;
END$$;

DO $$
DECLARE skovvej road.id%TYPE;
BEGIN
    INSERT INTO road (road_name)
    VALUES ("skovvej")
    RETURNING id INTO skovvej;
END$$;

DO $$
DECLARE stationsvej road.id%TYPE;
BEGIN
    INSERT INTO road (road_name)
    VALUES ("stationsvej")
    RETURNING id INTO stationsvej;
END$$;

DO $$
DECLARE elmevej road.id%TYPE;
BEGIN
    INSERT INTO road (road_name)
    VALUES ("elmevej")
    RETURNING id INTO elmevej;
END$$;

DO $$
DECLARE fasanvej road.id%TYPE;
BEGIN
    INSERT INTO road (road_name)
    VALUES ("fasanvej")
    RETURNING id INTO fasanvej;
END$$;
