use crate::{
    score_tracker::LoadScoreTrackerParameters, Error, NewPlayerParameters, Player,
    PlayerScore, Score, ScoreDetails, ScoreTracker,
};
use uuid::Uuid;

const PLAYERS_NUMBER: usize = 2;
const POINTS_LIMIT: u16 = 301;

pub struct GameState {
    game_id: Uuid,
    score_details: Vec<ScoreDetails>,
    score_tracker: ScoreTracker,
}

pub struct Round {
    pub number: u8,
    pub player_scores: Vec<PlayerScore>,
}

pub struct LoadGameStateParameters {
    pub game_id: Uuid,
    pub score_details: Vec<ScoreDetails>,
}

impl GameState {
    pub fn game_id(&self) -> Uuid {
        self.game_id
    }

    pub fn last_score_detail(&self) -> Option<&ScoreDetails> {
        self.score_details.last()
    }

    pub fn load(parameters: LoadGameStateParameters) -> Result<Self, Error> {
        let LoadGameStateParameters {
            game_id,
            score_details,
        } = parameters;

        let mut players: Vec<Player> = Vec::with_capacity(PLAYERS_NUMBER);

        for detail in score_details.iter() {
            if let Some(player) = players.get_mut(i32::from(detail.player_number()) as usize) {
                player.add_player_score(detail.player_score());
            } else {
                let mut player = Player::new(NewPlayerParameters {
                    number: i32::from(detail.player_number()) as usize,
                    points_limit: POINTS_LIMIT,
                });
                player.add_player_score(detail.player_score());
                players.push(player);
            }
        }

        let score_tracker = ScoreTracker::load(LoadScoreTrackerParameters {
            players_number: PLAYERS_NUMBER,
            points_limit: POINTS_LIMIT,
            players,
        });

        Ok(Self {
            game_id,
            score_details,
            score_tracker,
        })
    }

    pub fn new_turn(&mut self, score: Score) -> &Player {
        self.score_tracker.track(score)
    }

    pub fn player(&self) -> &Player {
        self.score_tracker.player()
    }

    pub fn players(&self) -> &[Player] {
        self.score_tracker.players()
    }

    pub fn pop_score_detail(mut self) -> Result<Self, Error> {
        self.score_details.pop();
        self.reload()
    }

    pub fn push_score_details(mut self, score_details: ScoreDetails) -> Result<Self, Error> {
        self.score_details.push(score_details);
        self.reload()
    }

    fn reload(self) -> Result<Self, Error> {
        Self::load(LoadGameStateParameters {
            game_id: self.game_id,
            score_details: self.score_details,
        })
    }

    // pub fn rounds(&self) -> Vec<Round> {
    //     let players_number = PlayerNumber::all().len();

    //     //TODO: calculate capacity in a more efficient way.
    //     let mut rounds = Vec::with_capacity(50);
    //     let mut round_number = 1;

    //     for chunk in self.score_details.chunks(players_number) {
    //         let mut round = Round {
    //             number: round_number,
    //             player_scores: Vec::with_capacity(players_number),
    //         };

    //         for data in chunk {
    //             round.player_scores.push(data.player_score());
    //         }

    //         rounds.push(round);
    //         round_number += 1;
    //     }

    //     rounds
    // }

    pub fn winner(&self) -> Option<&Player> {
        self.score_tracker.winner()
    }
}
