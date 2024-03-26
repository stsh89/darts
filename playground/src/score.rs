use crate::{Error, Points};

const POINTS_LIMIT: u16 = 180;

// #[derive(Clone, Copy)]
pub struct Score(Points);

impl Score {
    fn assign_points(&mut self, points: Points) -> Result<(), Error> {
        if points > Points::new(POINTS_LIMIT) {
            return Err(Error::InvalidArgument(format!(
                "The maximum number of points allowed is {POINTS_LIMIT}. Given: {points}"
            )));
        }

        self.0 = points;

        Ok(())
    }

    pub fn new(points: u16) -> Result<Self, Error> {
        let mut score = Score(Points::zero());

        score.assign_points(Points::new(points))?;

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
        let score = Score::new(0);

        assert!(score.is_ok());
    }

    #[test]
    fn test_new_score_with_limit() {
        let score = Score::new(POINTS_LIMIT);

        assert!(score.is_ok());
    }

    #[test]
    fn test_new_score_over_limit() {
        let result = Score::new(POINTS_LIMIT + 1);

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
}
