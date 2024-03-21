use crate::Score;
use std::ops::Add;

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
