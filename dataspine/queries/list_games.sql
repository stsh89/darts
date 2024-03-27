SELECT
    end_time,
    id,
    players_number,
    points_limit,
    rounds as "rounds!: Json<Vec<RoundsColumn>>",
    start_time
FROM playground.games
ORDER BY insert_time DESC
LIMIT 10
