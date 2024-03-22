use crate::{NewPlayerParameters, Player, Points, Score};
use std::ops::Add;

pub struct ScoreTracker {
    players_number: usize,
    players: Vec<Player>,
    points_limit: Points,
}

pub struct NewScoreTrackerParameters {
    pub players_number: usize,
    pub points_limit: u16,
}

impl ScoreTracker {
    fn get_player(&self, number: usize) -> Player {
        self.players
            .get(number)
            .cloned()
            .unwrap_or_else(|| self.new_player(number))
    }

    pub fn new(parameters: NewScoreTrackerParameters) -> Self {
        let NewScoreTrackerParameters {
            players_number,
            points_limit,
        } = parameters;

        ScoreTracker {
            players_number,
            players: vec![],
            points_limit: Points::from(points_limit),
        }
    }

    fn new_player(&self, number: usize) -> Player {
        Player::new(NewPlayerParameters {
            number,
            points_limit: self.points_limit,
        })
    }

    pub fn player(&self) -> Player {
        self.players()
            .into_iter()
            .min_by(|a, b| a.scores().len().cmp(&b.scores().len()))
            .unwrap_or(Player::new(NewPlayerParameters {
                number: 0,
                points_limit: self.points_limit,
            }))
    }

    fn players(&self) -> Vec<Player> {
        let mut players = Vec::with_capacity(self.players_number);

        for number in 0..self.players_number {
            players.push(self.get_player(number));
        }

        players
    }

    pub fn track(&mut self, score: Score) {
        let player_number = self.player().number();

        let Some(player) = self.players.get_mut(player_number) else {
            self.track_first_score(score);

            return;
        };

        player.add_score(score);
    }

    fn track_first_score(&mut self, score: Score) {
        let mut player = Player::new(NewPlayerParameters {
            number: self.player().number(),
            points_limit: self.points_limit,
        });

        player.add_score(score);

        self.players.push(player);
    }

    pub fn winner(&self) -> Option<&Player> {
        self.players.iter().find(|p| p.is_winner())
    }
}

pub trait AddScore {
    fn add_score(&mut self, score: Score, game_score: &GameScore) -> PlayerScore;
}

pub trait TotalGameScore {
    fn total_game_score(self) -> GameScore;
}

#[derive(PartialEq)]
pub struct GameScore(u16);

#[derive(Clone, Copy)]
pub enum PlayerScore {
    Score(Score),
    Overthrow(Score),
}

impl PlayerScore {
    pub fn score(points: u16) -> Self {
        PlayerScore::Score(Score::try_from(points).unwrap())
    }

    pub fn overthrow(points: u16) -> Self {
        PlayerScore::Overthrow(Score::try_from(points).unwrap())
    }

    pub fn into_inner(&self) -> u16 {
        match self {
            PlayerScore::Score(score) => score.points().into(),
            PlayerScore::Overthrow(score) => score.points().into(),
        }
    }

    pub fn points(&self) -> Points {
        match self {
            PlayerScore::Score(score) => score,
            PlayerScore::Overthrow(score) => score,
        }
        .points()
    }

    pub fn is_score(&self) -> bool {
        matches!(self, PlayerScore::Score(_))
    }
}

impl GameScore {
    pub fn new(x: u16) -> GameScore {
        GameScore(x)
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}

impl std::fmt::Display for GameScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AddScore for Vec<PlayerScore> {
    fn add_score(&mut self, score: Score, game_score: &GameScore) -> PlayerScore {
        let player_game_score: GameScore = self.iter().total_game_score();

        let player_score =
            if (player_game_score + GameScore(score.points().into())).0 > game_score.0 {
                PlayerScore::Overthrow(score)
            } else {
                PlayerScore::Score(score)
            };

        self.push(player_score);

        player_score
    }
}

impl<'a, T> TotalGameScore for T
where
    T: Iterator<Item = &'a PlayerScore>,
{
    fn total_game_score(self) -> GameScore {
        self.fold(GameScore(0), |acc, x| match x {
            PlayerScore::Score(score) => GameScore(score.points().into()) + acc,
            PlayerScore::Overthrow(_score) => acc,
        })
    }
}

impl Add for GameScore {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        GameScore(self.0 + rhs.0)
    }
}
