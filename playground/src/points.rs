use std::{fmt::Display, ops::Add};

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Points(u16);

impl Add for Points {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Display for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u16> for Points {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<Points> for u16 {
    fn from(value: Points) -> Self {
        value.0
    }
}

impl From<Points> for i32 {
    fn from(value: Points) -> Self {
        value.0.into()
    }
}
