INSERT INTO road (road_name)
VALUES
    ('lærkevej'),
    ('birkevej'),
    ('vibevej'),
    ('vinkelvej'),
    ('søndergade'),
    ('bakkevej'),
    ('nørregade'),
    ('drosselvej'),
    ('skovvej'),
    ('stationsvej'),
    ('elmevej'),
    ('fasanvej');

INSERT INTO road_part (road_id)
VALUES
    (1), (1), (1), -- lærkevej - 1,2,3
    (2), (2), (2), -- birkevej - 4,5,6
    (3), (3), (3), (3), -- vibevej - 7,8,9,10
    (4), (4), -- vinkelvej - 11,12
    (5), (5), (5), (5), (5), -- søndergade - 13,14,15,16,17
    (6), (6), -- bakkevej - 18,19
    (7), (7), (7), (7), (7), (7), -- nørregade - 20,21,22,23,24,25
    (8), -- drosselvej - 26
    (9), (9), (9), -- skovvej - 27,28,29
    (10), -- stationsvej - 30
    (11), (11), -- elmevej - 31,32
    (12), (12), (12), (12), (12), (12), (12); -- fasanvej - 33,34,35,36,37,38,39

INSERT INTO road_part_relation (part_one, part_two, weight)
VALUES
    -- link parts of roads together START
    (1, 2, 1), (2, 3, 2), -- lærkevej
    (4, 5, 5), (5, 6, 2), -- birkevej
    (7, 8, 3), (8, 9, 2), (9, 10, 4), -- vibevej
    (11, 12, 11), -- vinkelvel
    (13, 14, 2), (14, 15, 3), (15, 16, 6), (16, 17, 4), -- søndergade
    (18, 19, 2), -- bakevej
    (20, 21, 4), (21, 22, 2), (22, 23, 8), (23, 24, 2), (24, 25, 9), -- nørregade
    (27, 28, 3), (28, 29, 6), -- skovvej
    (31, 32, 10), -- elmevej
    (33, 34, 2), (34, 35, 3), (35, 36, 9), (36, 37, 3), (37, 38, 2), (38, 39, 2), -- fasanvej
    -- link parts of roads together END

    -- link different roads together START
    (1, 4, 3),
    (1, 6, 4),
    (3, 9, 4),
    (6, 11, 1),
    (11, 18, 9),
    (3, 13, 5),
    (10, 14, 2),
    (15, 20, 1),
    (24, 26, 5),
    (5, 27, 8),
    (28, 10, 1),
    (28, 33, 1),
    (34, 12, 3),
    (19, 36, 8);
    -- link different roads together END

-- Visual of the end result (atleast the links. The relations with bigger weights will be larger roads)
/*
*                                                    26
*                                                     |
* 7 -- 8 -- 9 -- 10          20 -- 21 -- 22 -- 23 -- 24 -- 25
* |         |        \        |
* 1 -- 2 -- 3 -- 13 -- 14 -- 15 -- 16 -- 17
* |
* 4 -- 5 -- 6 -- 11 -- 18 -- 19
*      |          |           |
*      27        12           |
*      |          |           |
* 10 - 28 -- 33 -- 34 -- 35 -- 36 -- 37 -- 38 -- 39
*      |
*      29
* */
