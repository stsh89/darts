use uuid::Uuid;

use crate::{
    AddScore, Error, GameScore, PlayerNumber, PlayerScore, Score, ScoreDetails, TotalGameScore,
};

pub struct GameState {
    game_id: Uuid,
    score_details: Vec<ScoreDetails>,
}

pub struct PlayerState {
    player_number: PlayerNumber,
    points_to_win: GameScore,
}

pub struct Round {
    pub number: u8,
    pub player_scores: Vec<PlayerScore>,
}

pub struct Turn {
    pub round_number: u8,
    pub player_number: PlayerNumber,
    pub player_score: PlayerScore,
}

pub struct LoadGameStateParameters {
    pub game_id: Uuid,
    pub score_details: Vec<ScoreDetails>,
}

impl GameState {
    pub fn current_player_state(&self) -> PlayerState {
        let Some(details) = self.score_details.last() else {
            return PlayerState::new(PlayerNumber::One, self.max_game_score());
        };

        let player_number = details.player_number().next_player_number();

        PlayerState::new(player_number, self.player_points_to_win(player_number))
    }

    fn current_player_number(&self) -> PlayerNumber {
        self.score_details
            .last()
            .map(|details| details.player_number().next_player_number())
            .unwrap_or(PlayerNumber::One)
    }

    pub fn game_id(&self) -> Uuid {
        self.game_id
    }

    pub fn load(parameters: LoadGameStateParameters) -> Result<Self, Error> {
        let LoadGameStateParameters {
            game_id,
            score_details,
        } = parameters;

        Ok(Self {
            game_id,
            score_details,
        })
    }

    fn max_game_score(&self) -> GameScore {
        GameScore::new(301)
    }

    pub fn new_turn(&self, score: Score) -> Result<Turn, Error> {
        let player_number = self.current_player_number();
        let mut player_scores: Vec<PlayerScore> = self
            .score_details
            .iter()
            .filter(|details| details.player_number() == player_number)
            .map(|details| details.player_score())
            .collect();

        let player_score = player_scores.add_score(score, &self.max_game_score());

        let round_number = if matches!(player_number, PlayerNumber::One) {
            self.rounds().len() + 1
        } else {
            self.rounds().len()
        }
        .try_into()
        .map_err(Into::<eyre::Report>::into)?;

        Ok(Turn {
            round_number,
            player_number,
            player_score,
        })
    }

    fn player_game_score(&self, player_number: PlayerNumber) -> GameScore {
        self.score_details
            .iter()
            .filter(|details| details.player_number() == player_number)
            .map(|details| details.player_score())
            .collect::<Vec<PlayerScore>>()
            .iter()
            .total_game_score()
    }

    pub fn players_game_scores(&self) -> Vec<PlayerState> {
        PlayerNumber::all()
            .into_iter()
            .map(|player_number| {
                PlayerState::new(player_number, self.player_points_to_win(player_number))
            })
            .collect()
    }

    //TODO: ensure substract operation correctness
    fn player_points_to_win(&self, player_number: PlayerNumber) -> GameScore {
        let value = self.max_game_score().value() - self.player_game_score(player_number).value();

        GameScore::new(value)
    }

    pub fn pop_score_details(&mut self) -> Option<ScoreDetails> {
        self.score_details.pop()
    }

    pub fn push_score_details(&mut self, score_details: ScoreDetails) {
        self.score_details.push(score_details);
    }

    pub fn rounds(&self) -> Vec<Round> {
        let players_number = PlayerNumber::all().len();

        //TODO: calculate capacity in a more efficient way.
        let mut rounds = Vec::with_capacity(50);
        let mut round_number = 1;

        for chunk in self.score_details.chunks(players_number) {
            let mut round = Round {
                number: round_number,
                player_scores: Vec::with_capacity(players_number),
            };

            for data in chunk {
                round.player_scores.push(data.player_score());
            }

            rounds.push(round);
            round_number += 1;
        }

        rounds
    }

    pub fn winner(&self) -> Option<PlayerNumber> {
        let max_game_score = self.max_game_score();

        for player_number in PlayerNumber::all() {
            let score = self.player_game_score(player_number);

            if score == max_game_score {
                return Some(player_number);
            }
        }

        None
    }
}

impl PlayerState {
    fn new(player_number: PlayerNumber, points_to_win: GameScore) -> Self {
        Self {
            player_number,
            points_to_win,
        }
    }

    pub fn player_number(&self) -> PlayerNumber {
        self.player_number
    }

    pub fn points_to_win(&self) -> &GameScore {
        &self.points_to_win
    }
}
