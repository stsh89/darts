use crate::{Error, Points};

/// Player can not score more than 180 points at a time.
/// Maximum points per one dart is 60.
/// Player throws 3 darts per one round which is equal to 180 points.
const POINTS_LIMIT: u16 = 180;

pub struct Score(Points);

impl Score {
    pub fn new(points: u16) -> Result<Self, Error> {
        ScoreBuilder::new().points(Points::new(points)).build()
    }

    pub fn points(&self) -> Points {
        self.0
    }
}

struct ScoreBuilder {
    points: Result<Points, Error>,
}

impl ScoreBuilder {
    fn new() -> Self {
        Self {
            points: Err(Error::InvalidArgument("Missing score points".to_string())),
        }
    }

    fn points(mut self, points: Points) -> Self {
        self.points = if points > Points::new(POINTS_LIMIT) {
            let description = format!(
                "The maximum number of points allowed is {}. Given: {}",
                POINTS_LIMIT, points
            );

            Error::invalid_argument(description).into()
        } else {
            Ok(points)
        };

        self
    }

    fn build(self) -> Result<Score, Error> {
        let points = self.points?;

        Ok(Score(points))
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
