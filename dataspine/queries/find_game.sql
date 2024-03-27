SELECT
    end_time,
    id,
    players_number,
    points_limit,
    rounds as "rounds!: Json<Vec<RoundsColumn>>",
    start_time
FROM playground.games
WHERE id = $1
