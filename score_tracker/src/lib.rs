use std::ops::Sub;

pub trait AddScore {
    fn add_score(&mut self, score: Score, game_score: &GameScore);
}

pub trait TotalGameScore {
    fn total_game_score(self) -> GameScore;
}

pub struct Score(u8);

pub struct GameScore(u16);

pub enum PlayerScore {
    Score(Score),
    Overflow(Score),
}

impl Score {
    pub fn new(x: u8) -> Score {
        Score(x)
    }
}

impl GameScore {
    pub fn new(x: u16) -> GameScore {
        GameScore(x)
    }
}

impl std::fmt::Display for GameScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a, 'b> Sub<&'b GameScore> for &'a GameScore {
    type Output = GameScore;

    fn sub(self, rhs: &'b GameScore) -> Self::Output {
        GameScore(self.0 - rhs.0)
    }
}

impl AddScore for Vec<PlayerScore> {
    fn add_score(&mut self, score: Score, game_score: &GameScore) {
        add_score(self, score, game_score);
    }
}

fn add_score(scores: &mut Vec<PlayerScore>, score: Score, game_score: &GameScore) {
    let player_game_score: GameScore = scores.iter().total_game_score();

    if (player_game_score.0 + Into::<u16>::into(score.0)) > game_score.0 {
        scores.push(PlayerScore::Overflow(score));
    } else {
        scores.push(PlayerScore::Score(score));
    }
}

impl PartialEq for GameScore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for GameScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<'a, T> TotalGameScore for T
where
    T: Iterator<Item = &'a PlayerScore>,
{
    fn total_game_score(self) -> GameScore {
        total_game_score(self)
    }
}

fn total_game_score<'a, I: Iterator<Item = &'a PlayerScore>>(iter: I) -> GameScore {
    let acc = iter.fold(0u16, |acc, x| match x {
        PlayerScore::Score(score) => Into::<u16>::into(score.0) + acc,
        PlayerScore::Overflow(_score) => acc,
    });

    GameScore(acc)
}
