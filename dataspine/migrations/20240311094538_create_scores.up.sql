CREATE TABLE playground.scores (
    game_id uuid NOT NULL,
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    insert_time timestamp(6) WITH time ZONE NOT NULL DEFAULT now(),
    player_number int NOT NULL,
    points_kind varchar NOT NULL,
    points_number int NOT NULL,
    round_number int NOT NULL
)
