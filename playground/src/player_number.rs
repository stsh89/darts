use crate::Error;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum PlayerNumber {
    One,
    Two,
}

impl PlayerNumber {
    pub fn all() -> Vec<Self> {
        vec![PlayerNumber::One, PlayerNumber::Two]
    }

    pub fn name(&self) -> &str {
        match self {
            PlayerNumber::One => "Player1",
            PlayerNumber::Two => "Player2",
        }
    }

    pub fn next_player_number(self) -> Self {
        match self {
            PlayerNumber::One => PlayerNumber::Two,
            PlayerNumber::Two => PlayerNumber::One,
        }
    }
}

impl TryFrom<i32> for PlayerNumber {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value == 1 {
            return Ok(PlayerNumber::One);
        }

        if value == 2 {
            return Ok(PlayerNumber::Two);
        }

        Err(Error::Unexpected(eyre::eyre!(
            "Invalid player number: {value}"
        )))
    }
}

impl From<PlayerNumber> for i32 {
    fn from(value: PlayerNumber) -> Self {
        match value {
            PlayerNumber::One => 1,
            PlayerNumber::Two => 2,
        }
    }
}
