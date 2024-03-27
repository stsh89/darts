use clap::Parser;
use dataspine::Repo;
use playground::{
    coordinator::{self, CountScoreParameters, InitializeGameParameters},
    InProgressState, NotStartedState, Number, Points, Score, State,
};
use std::io;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    database_url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Args { database_url } = Args::parse();
    let repo = Repo::from_database_url(&database_url).await?;

    let mut game = coordinator::initialize_game(InitializeGameParameters {
        games: &repo,
        players_number: Number::new(1)?,
        points_limit: Points::new(101),
    })
    .await?;

    loop {
        let prompt = match game.state() {
            State::NotStarted(state) => Prompt::from(state),
            State::InProgress(state) => Prompt::from(state),
            State::Finished(state) => {
                println!("\nPlayer{} won", state.winner());
                return Ok(());
            }
        };

        display_prompt(&prompt);

        let score = match get_score() {
            Ok(score) => score,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        game = match coordinator::count_score(CountScoreParameters {
            game_id: game.id().unwrap(),
            games: &repo,
            score,
        })
        .await
        {
            Ok(game) => game,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
    }
}

struct Prompt {
    player_number: Number,
    points_to_win: Points,
}

impl From<&NotStartedState> for Prompt {
    fn from(value: &NotStartedState) -> Self {
        Self {
            player_number: value.player_number(),
            points_to_win: value.points_to_win(),
        }
    }
}

impl From<&InProgressState> for Prompt {
    fn from(value: &InProgressState) -> Self {
        Self {
            player_number: value.player_number(),
            points_to_win: value.points_to_win(),
        }
    }
}

fn display_prompt(prompt: &Prompt) {
    let Prompt {
        player_number,
        points_to_win,
    } = prompt;

    println!("\nPlayer{} {}", player_number, points_to_win);
    println!("Enter score: ");
}

fn get_score() -> anyhow::Result<Score> {
    let mut input_line = String::new();
    let Ok(_) = io::stdin().read_line(&mut input_line) else {
        anyhow::bail!("Error reading input");
    };

    let Ok(x): Result<u16, _> = input_line.trim().parse() else {
        anyhow::bail!("Ensure to enter a score between 0 and 180");
    };

    let score = Score::new(x)?;

    Ok(score)
}
