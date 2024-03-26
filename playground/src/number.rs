use crate::Error;
use std::{fmt::Display, num::NonZeroUsize};

#[derive(Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Number(NonZeroUsize);

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Number> for i32 {
    fn from(value: Number) -> Self {
        value.0.get() as i32
    }
}

impl Number {
    pub(crate) unsafe fn new_unchecked(value: usize) -> Self {
        Number(NonZeroUsize::new_unchecked(value))
    }

    pub fn new(value: usize) -> Result<Self, Error> {
        let value = NonZeroUsize::new(value).ok_or(Error::InvalidArgument(
            "Number must be greater than 0".to_string(),
        ))?;

        Ok(Number(value))
    }

    pub fn one() -> Self {
        Number(unsafe { NonZeroUsize::new_unchecked(1) })
    }

    pub fn value(&self) -> usize {
        self.0.get()
    }

    pub fn increment(&mut self) {
        self.0 = self.0.saturating_add(1);
    }
}
