INSERT INTO playground.games (
    end_time,
    players_number,
    points_limit,
    rounds,
    start_time
) VALUES ($1, $2, $3, $4, $5)
RETURNING id, insert_time, update_time
