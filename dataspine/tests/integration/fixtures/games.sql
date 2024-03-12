WITH game(id) AS (
    INSERT INTO playground.games DEFAULT VALUES RETURNING id
)
INSERT INTO playground.scores (game_id, player_name, score, turn_number)
SELECT game.id, T.player_name, T.score, T.turn_number
FROM game
LEFT JOIN (
    SELECT player_name, score, turn_number FROM UNNEST(
        array['Player1', 'Player2', 'Player1', 'Player2'],
        array[17, 55, 34, 66],
        array[1,1,2,2]
    ) AS VALUES(player_name, score, turn_number)
) T ON TRUE;
