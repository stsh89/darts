WITH game(id) AS (
    INSERT INTO playground.games DEFAULT VALUES RETURNING id
)
INSERT INTO playground.scores (game_id, player_number, points_kind, points_number, round_number)
SELECT game.id, T.player_number, T.points_kind, T.points_number, T.round_number
FROM game
LEFT JOIN (
    SELECT player_number, points_kind, points_number, round_number FROM UNNEST(
        array[1, 2, 1, 2],
        array['score', 'score', 'score', 'score'],
        array[17, 55, 34, 66],
        array[1,1,2,2]
    ) AS VALUES(player_number, points_kind, points_number, round_number)
) T ON TRUE;
