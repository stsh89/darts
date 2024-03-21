use playground::{NewScoreTrackerParameters, Score, ScoreTracker};
use std::io;

fn main() {
    let mut input_line = String::with_capacity(20);

    let mut score_tracker = ScoreTracker::new(NewScoreTrackerParameters {
        players_number: 2,
        points_limit: 301,
    });

    loop {
        println!(
            "Player{} {}",
            score_tracker.player_to_score() + 1,
            score_tracker.player_to_score_points_to_win()
        );

        println!("Enter score: ");

        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        if input_line == "exit\n" {
            break;
        }

        let x: u16 = input_line.trim().parse().expect("Input not an integer");
        input_line = String::new();
        let score = Score::try_from(x).expect("Max 180 points allowed");

        score_tracker.track(score);

        println!();

        if let Some(winner) = score_tracker.winner() {
            println!("Player{} won", winner + 1);

            break;
        }
    }
}
