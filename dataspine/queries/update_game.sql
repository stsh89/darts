UPDATE playground.games
SET
    end_time = $2,
    players_number = $3,
    points_limit = $4,
    rounds = $5,
    start_time = $6,
    update_time = default
WHERE id = $1
