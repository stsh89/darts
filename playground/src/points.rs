use std::{fmt::Display, iter::Sum, ops::Add};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
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

impl Sum for Points {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl Points {
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}
