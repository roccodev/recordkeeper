use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FiniteF32(f32);

#[derive(Clone, Copy, Debug)]
pub struct FromFloatError;

#[derive(Clone, Debug)]
pub enum FromStrError {
    Parse(<f32 as FromStr>::Err),
    Float(FromFloatError),
}

impl TryFrom<f32> for FiniteF32 {
    type Error = FromFloatError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        value
            .is_finite()
            .then_some(Self(value))
            .ok_or(FromFloatError)
    }
}

impl Ord for FiniteF32 {
    fn cmp(&self, other: &Self) -> Ordering {
        let Some(ord) = self.0.partial_cmp(&other.0) else {
            unreachable!()
        };
        ord
    }
}

impl PartialOrd for FiniteF32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for FiniteF32 {}

impl Display for FiniteF32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for FiniteF32 {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        f32::from_str(s)
            .map_err(FromStrError::Parse)
            .and_then(|f| f.try_into().map_err(FromStrError::Float))
    }
}

impl From<FiniteF32> for f32 {
    fn from(value: FiniteF32) -> Self {
        value.0
    }
}
