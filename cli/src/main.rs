use playground::{NewScoreTrackerParameters, Score, ScoreTracker};
use std::io;

fn main() -> anyhow::Result<()> {
    let mut score_tracker = ScoreTracker::new(NewScoreTrackerParameters {
        players_number: 2,
        points_limit: 301,
    });

    loop {
        let mut input_line = String::new();

        println!(
            "\nPlayer{} {}",
            score_tracker.player_to_score() + 1,
            score_tracker.player_to_score_points_to_win()
        );

        println!("Enter score: ");

        let Ok(_) = io::stdin().read_line(&mut input_line) else {
            continue;
        };

        let Ok(x): Result<u16, _> = input_line.trim().parse() else {
            println!("Ensure to enter a score between 0 and 180");
            continue;
        };

        let score = match Score::try_from(x) {
            Ok(score) => score,
            Err(err) => {
                println!("{err}");
                continue;
            }
        };

        score_tracker.track(score);

        if let Some(winner) = score_tracker.winner() {
            println!("\nPlayer{} won", winner + 1);

            return Ok(());
        }
    }
}
