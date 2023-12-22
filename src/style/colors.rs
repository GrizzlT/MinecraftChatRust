#[cfg(not(feature = "palette"))]
use crate::freeze::FrozenStr;

#[cfg(feature = "palette")]
mod rgb {
    use std::fmt::Display;
    use std::hash::{Hash, Hasher};
    use std::str::FromStr;
    use palette::rgb::FromHexError;
    use palette::Srgb;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Rgb(pub palette::rgb::Rgb<Srgb, u8>);

    impl Hash for Rgb {
        fn hash<H: Hasher>(&self, state: &mut H) {
            state.write_u8(self.0.red);
            state.write_u8(self.0.green);
            state.write_u8(self.0.blue);
        }
    }

    impl FromStr for Rgb {
        type Err = FromHexError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(s.parse()?))
        }
    }

    impl Display for Rgb {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", format!("#{:02x}{:02x}{:02x}", self.0.red, self.0.green, self.0.blue))
        }
    }
}

#[cfg(feature = "palette")]
pub use self::rgb::*;

/// The different colors a [`Chat`] component can have.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TextColor {
    Black,
    DarkBlue,
    DarkGreen,
    DarkCyan,
    DarkRed,
    Purple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Cyan,
    Red,
    Pink,
    Yellow,
    White,
    /// This field is ignored for versions older than 1.16.
    ///
    /// See [`TextColor::custom()`].
    #[cfg(not(feature = "palette"))]
    Custom(FrozenStr),
    /// This field is ignored for versions older than 1.16.
    #[cfg(feature = "palette")]
    Custom(Rgb),
    Reset,
}

#[cfg(not(feature = "palette"))]
impl TextColor {
    pub fn custom<T: Into<FrozenStr>>(color: T) -> TextColor {
        TextColor::Custom(color.into())
    }
}