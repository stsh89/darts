SELECT
    end_time,
    id,
    insert_time,
    players_number,
    points_limit,
    rounds as "rounds!: Json<Vec<RoundsColumnItem>>",
    start_time,
    update_time
FROM playground.games
WHERE id = $1
