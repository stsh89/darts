use crate::Error;
use std::{fmt::Display, num::NonZeroUsize};

#[derive(Clone, Copy, PartialEq)]
pub struct Number(NonZeroUsize);

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
}
