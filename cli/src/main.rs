use playground::score_tracker::{AddScore, GameScore, Score, TotalGameScore};
use std::io;

fn main() {
    let mut input_line = String::with_capacity(20);
    let mut player1_scores = Vec::with_capacity(20);
    let mut player2_scores = Vec::with_capacity(20);
    let mut player_number = 1;
    let max_score = GameScore::new(301);

    loop {
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        if input_line == "exit\n" {
            break;
        }

        println!("{}", &input_line);

        let x: u8 = input_line.trim().parse().expect("Input not an integer");
        input_line = String::new();

        if player_number == 1 {
            player1_scores.add_score(Score::new(x), &max_score);
            let total = player1_scores.iter().total_game_score();
            println!("TOTALITY {}", &total);
            println!("Player1\ntotal: {}, left: {}", &total, &max_score - &total);

            if total == max_score {
                break;
            } else {
                player_number = 2;
            }
        } else {
            player2_scores.add_score(Score::new(x), &max_score);
            let total = player2_scores.iter().total_game_score();
            println!("Player2\ntotal: {}, left: {}", &total, &max_score - &total);

            if total == max_score {
                break;
            } else {
                player_number = 1;
            }
        };
    }
}
