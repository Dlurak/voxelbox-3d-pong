#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Odd<T>(T);

macro_rules! impl_odd {
    ($ty:ty) => {
        #[allow(dead_code)]
        impl Odd<$ty> {
            pub const fn new(value: $ty) -> Option<Self> {
                if value % 2 == 1 {
                    Some(Self(value))
                } else {
                    None
                }
            }
            pub const fn new_panics(value: $ty) -> Self {
                assert!(value % 2 != 0, "The number must be odd!");
                Self(value)
            }

            pub const fn value(&self) -> $ty {
                self.0
            }
        }

        impl Default for Odd<$ty> {
            fn default() -> Self {
                Self(1)
            }
        }

        impl From<Odd<$ty>> for $ty {
            fn from(value: Odd<$ty>) -> Self {
                value.0
            }
        }

        impl TryFrom<$ty> for Odd<$ty> {
            type Error = ();
            fn try_from(value: $ty) -> Result<Self, Self::Error> {
                if value % 2 == 1 {
                    Ok(Self(value))
                } else {
                    Err(())
                }
            }
        }

        impl std::ops::Add<$ty> for Odd<$ty> {
            type Output = $ty;

            fn add(self, rhs: $ty) -> Self::Output {
                self.0 + rhs
            }
        }

        impl std::ops::Sub<$ty> for Odd<$ty> {
            type Output = $ty;

            fn sub(self, rhs: $ty) -> Self::Output {
                self.0 - rhs
            }
        }

        impl std::ops::Mul<$ty> for Odd<$ty> {
            type Output = $ty;

            fn mul(self, rhs: $ty) -> Self::Output {
                self.0 * rhs
            }
        }

        impl std::ops::Div<$ty> for Odd<$ty> {
            type Output = $ty;

            fn div(self, rhs: $ty) -> Self::Output {
                self.0 / rhs
            }
        }
    };
}

impl_odd!(u8);
impl_odd!(u16);
impl_odd!(u32);
impl_odd!(u64);
impl_odd!(u128);
impl_odd!(usize);

impl_odd!(i8);
impl_odd!(i16);
impl_odd!(i32);
impl_odd!(i64);
impl_odd!(i128);
impl_odd!(isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd() {
        assert_eq!(Odd::<u32>::new(3), Some(Odd(3 as u32)));
        assert_eq!(Odd::<u32>::new(2), None);
        assert_eq!(Odd::<u32>::new_panics(3).value(), 3);
    }
}
