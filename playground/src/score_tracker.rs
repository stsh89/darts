pub trait AddScore {
    fn add_score(&mut self, score: Score, game_score: &GameScore) -> PlayerScore;
}

pub trait TotalGameScore {
    fn total_game_score(self) -> GameScore;
}

#[derive(Clone, Copy)]
pub struct Score(u8);

#[derive(PartialEq)]
pub struct GameScore(u16);

#[derive(Clone, Copy)]
pub enum PlayerScore {
    Score(Score),
    Overthrow(Score),
}

impl Score {
    pub fn new(x: u8) -> Score {
        Score(x)
    }

    pub fn points(&self) -> u8 {
        self.0
    }
}

impl PlayerScore {
    pub fn score(points: u8) -> Self {
        PlayerScore::Score(Score::new(points))
    }

    pub fn overthrow(points: u8) -> Self {
        PlayerScore::Overthrow(Score::new(points))
    }

    pub fn into_inner(&self) -> u8 {
        match self {
            PlayerScore::Score(score) => score.points(),
            PlayerScore::Overthrow(score) => score.points(),
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

        let player_score = if (player_game_score.0 + Into::<u16>::into(score.0)) > game_score.0 {
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
        let acc = self.fold(0u16, |acc, x| match x {
            PlayerScore::Score(score) => Into::<u16>::into(score.0) + acc,
            PlayerScore::Overthrow(_score) => acc,
        });

        GameScore(acc)
    }
}
