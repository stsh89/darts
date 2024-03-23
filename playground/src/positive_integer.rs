use crate::Error;
use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
pub struct PositiveInteger(usize);

impl Display for PositiveInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Iterator for PositiveIntegersRange {
    type Item = PositiveInteger;

    fn next(&mut self) -> Option<Self::Item> {
        if self.begin == self.end.value() {
            return None;
        }

        self.begin += 1;

        Some(PositiveInteger(self.begin))
    }
}

impl TryFrom<i32> for PositiveInteger {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let value: usize = value.try_into().map_err(|_err| {
            Error::Unexpected(eyre::eyre!(
                "Can't convert i32 into usize. Given: {}",
                value
            ))
        })?;

        Self::new(value)
    }
}

impl TryFrom<PositiveInteger> for i32 {
    type Error = Error;

    fn try_from(value: PositiveInteger) -> Result<Self, Self::Error> {
        value.value().try_into().map_err(|_err| {
            Error::Unexpected(eyre::eyre!(
                "Can't convert positive integer into i32. Given: {}",
                value
            ))
        })
    }
}

pub struct PositiveIntegersRange {
    end: PositiveInteger,
    begin: usize,
}

impl PositiveInteger {
    pub fn new(value: usize) -> Result<Self, Error> {
        if value == 0 {
            return Err(Error::InvalidArgument(
                "Positive integer must be greater than 0".to_string(),
            ));
        }

        Ok(PositiveInteger(value))
    }

    pub fn one() -> Self {
        PositiveInteger(1)
    }

    pub fn range(&self) -> PositiveIntegersRange {
        PositiveIntegersRange {
            begin: 0,
            end: *self,
        }
    }

    pub fn try_to_i32(&self) -> Result<i32, Error> {
        self.0.try_into().map_err(|_| {
            Error::FailedPrecondition("Can't convert positive integer into i32".to_string())
        })
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_from_one() {
        let mut range = PositiveInteger::one().range();

        assert_eq!(Some(1), range.next().map(|i| i.value()));
        assert_eq!(None, range.next().map(|i| i.value()));
    }

    #[test]
    fn test_range() {
        let mut range = PositiveInteger(5).range();

        assert_eq!(Some(1), range.next().map(|i| i.value()));
        assert_eq!(Some(2), range.next().map(|i| i.value()));
        assert_eq!(Some(3), range.next().map(|i| i.value()));
        assert_eq!(Some(4), range.next().map(|i| i.value()));
        assert_eq!(Some(5), range.next().map(|i| i.value()));
        assert_eq!(None, range.next().map(|i| i.value()));
    }
}
