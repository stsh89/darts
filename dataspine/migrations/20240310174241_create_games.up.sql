CREATE TABLE playground.games (
    end_time timestamp(6) WITH time ZONE,
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    insert_time timestamp(6) WITH time ZONE NOT NULL DEFAULT now(),
    players_number int NOT NULL,
    points_limit int NOT NULL,
    rounds jsonb NOT NULL,
    start_time timestamp(6) WITH time ZONE,
    update_time timestamp(6) WITH time ZONE NOT NULL DEFAULT now()
);
