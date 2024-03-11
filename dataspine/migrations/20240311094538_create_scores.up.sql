CREATE TABLE playground.scores (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    game_id uuid NOT NULL,
    player_name varchar NOT NULL,
    score int NOT NULL,
    turn_number int NOT NULL,
    insert_time timestamp(6) WITH time ZONE NOT NULL DEFAULT now()
)
