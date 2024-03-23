use crate::{Error, Points};

#[derive(Clone, Copy)]
pub struct PointsLimit(Points);

impl TryFrom<u16> for PointsLimit {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(Points::from(value))
    }
}

impl TryFrom<Points> for PointsLimit {
    type Error = Error;

    fn try_from(value: Points) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl PointsLimit {
    pub fn new(points: Points) -> Result<Self, Error> {
        if points.is_zero() {
            return Err(Error::InvalidArgument(
                "Points limit cannot be zero".to_string(),
            ));
        }

        Ok(Self(points))
    }

    pub fn points(&self) -> Points {
        self.0
    }
}
