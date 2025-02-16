#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rgb(u8, u8, u8);

macro_rules! colors_by_name {
    ($($color:ident => ($red:expr, $green:expr, $blue:expr)),* $(,)?) => {
        impl Rgb {
            $(
                #[inline]
                pub const fn $color() -> Self {
                    Self($red, $green, $blue)
                }
            )*
        }
    };
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }
}

colors_by_name! {
    black => (0, 0, 0),
    green => (22, 163, 74),
    yellow => (234, 179, 8),
    pink => (219, 39, 119)
}

impl From<Rgb> for (u8, u8, u8) {
    #[inline]
    fn from(value: Rgb) -> Self {
        let Rgb(r, g, b) = value;
        (r, g, b)
    }
}
