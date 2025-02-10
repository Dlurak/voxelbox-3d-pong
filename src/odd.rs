#[derive(Clone, Copy, Debug, Hash)]
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
