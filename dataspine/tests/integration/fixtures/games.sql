INSERT INTO playground.games (
    players_number, points_limit, rounds, start_time
) VALUES (
    2,
    301,
    '[
        {"points": 17, "points_kind": "regular", "round_number": 1, "player_number": 1},
        {"points": 24, "points_kind": "regular", "round_number": 1, "player_number": 2},
        {"points": 27, "points_kind": "regular", "round_number": 2, "player_number": 1}
    ]'::jsonb,
    NOW()
)
