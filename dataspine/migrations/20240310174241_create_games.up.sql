CREATE TABLE playground.games (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    insert_time timestamp(6) WITH time ZONE NOT NULL DEFAULT now()
);
