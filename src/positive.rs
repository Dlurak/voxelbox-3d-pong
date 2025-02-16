use num_traits::identities::ConstZero;
use std::{
    cmp::PartialOrd,
    fmt,
    ops::{Add, Mul},
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Positive<T>(T);

impl<T: ConstZero + PartialOrd + Copy> Positive<T> {
    pub fn new(value: T) -> Option<Self> {
        if value > T::ZERO {
            Some(Self(value))
        } else {
            None
        }
    }

    pub const fn value(&self) -> T {
        self.0
    }
}

impl<T: Add<Output = T>> Add for Positive<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: Mul<Output = T>> Mul for Positive<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl<T: ConstZero + PartialOrd + Copy + fmt::Display> fmt::Display for Positive<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
