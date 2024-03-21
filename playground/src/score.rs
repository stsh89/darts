use crate::{Error, Points};

const POINTS_LIMIT: u16 = 180;

#[derive(Clone, Copy, Default)]
pub struct Score(Points);

impl TryFrom<Points> for Score {
    type Error = Error;

    fn try_from(value: Points) -> Result<Self, Self::Error> {
        Score::new(value)
    }
}

impl TryFrom<u16> for Score {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Points::from(value).try_into()
    }
}

impl TryFrom<i32> for Score {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let value = u16::try_from(value).map_err(|_err| {
            Error::InvalidArgument(format!(
                "Only points between 0 and {POINTS_LIMIT} are allowed. Given: {value}"
            ))
        })?;

        Points::from(value).try_into()
    }
}

impl Score {
    fn assign_points(&mut self, points: Points) -> Result<(), Error> {
        if Into::<u16>::into(points) > POINTS_LIMIT {
            return Err(Error::InvalidArgument(format!(
                "The maximum number of points allowed is {POINTS_LIMIT}. Given: {points}"
            )));
        }

        self.0 = points;

        Ok(())
    }

    fn new(points: Points) -> Result<Self, Error> {
        let mut score = Score::default();

        score.assign_points(points)?;

        Ok(score)
    }

    pub fn points(&self) -> Points {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_score() {
        let score = Score::try_from(0);

        assert!(score.is_ok());
    }

    #[test]
    fn test_new_score_with_points_limit() {
        let score = Score::try_from(POINTS_LIMIT);

        assert!(score.is_ok());
    }

    #[test]
    fn test_new_score_from_u16_over_limit() {
        let result = Score::try_from(POINTS_LIMIT + 1);

        assert!(result.is_err());

        match result {
            Err(Error::InvalidArgument(msg)) => {
                assert_eq!(
                    msg,
                    "The maximum number of points allowed is 180. Given: 181"
                )
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_new_score_from_i32() {
        let result = Score::try_from(-1);

        assert!(result.is_err());

        match result {
            Err(Error::InvalidArgument(msg)) => {
                assert_eq!(msg, "Only points between 0 and 180 are allowed. Given: -1")
            }
            _ => assert!(false),
        }
    }
}
