use playground::{Game, NewGameParameters, Number, Points, Score};
use std::io;

fn main() -> anyhow::Result<()> {
    let mut game = Game::new(NewGameParameters {
        players_number: Number::new(3)?,
        points_limit: Points::new(301),
    })?;

    loop {
        let Some(round_preview) = game.round_preview() else {
            println!("Game is over");
            println!(
                "Player{} won",
                game.winner()
                    .expect("Missing winner number but it is expected")
            );

            return Ok(());
        };

        println!(
            "\nPlayer{} {}",
            round_preview.player_number(),
            round_preview.points_to_win()
        );
        println!("Enter score: ");

        let mut input_line = String::new();
        let Ok(_) = io::stdin().read_line(&mut input_line) else {
            continue;
        };

        let Ok(x): Result<u16, _> = input_line.trim().parse() else {
            println!("Ensure to enter a score between 0 and 180");
            continue;
        };

        match Score::new(x) {
            Ok(score) => game.count_score(score)?,
            Err(err) => {
                println!("{err}");
                continue;
            }
        };
    }
}
