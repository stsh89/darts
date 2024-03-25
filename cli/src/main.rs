use playground::{NewScoreTrackerParameters, Number, PointsLimit, Score, ScoreTracker};
use std::io;

fn main() -> anyhow::Result<()> {
    let mut score_tracker = ScoreTracker::new(NewScoreTrackerParameters {
        players_number: Number::new(3)?,
        points_limit: PointsLimit::try_from(301)?,
    });

    loop {
        let mut input_line = String::new();

        let player = score_tracker.player();

        println!("\nPlayer{} {}", player.number(), player.points_to_win());

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

        let player = score_tracker.track(score);

        if player.is_winner() {
            println!("\nPlayer{} won", player.number());

            return Ok(());
        }
    }
}
