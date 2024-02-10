use std::convert::TryFrom;
use std::fmt::Display;
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
    pub struct Rgb(pub Srgb<u8>);

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

    impl From<(u8, u8, u8)> for Rgb {
        fn from(value: (u8, u8, u8)) -> Self {
            Self(palette::rgb::Rgb::from([value.0, value.1, value.2]))
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

/// The different colors a [`crate::Chat`] component can have.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "palette", derive(Copy))]
pub enum TextColor {
    /// RGB = (0, 0, 0)
    Black,
    /// RGB = (0, 0, 170)
    DarkBlue,
    /// RGB = (0, 170, 0)
    DarkGreen,
    /// RGB = (0, 170, 170)
    DarkCyan,
    /// RGB = (170, 0, 0)
    DarkRed,
    /// RGB = (170, 0, 170)
    Purple,
    /// RGB = (255, 170, 0)
    Gold,
    /// RGB = (170, 170, 170)
    Gray,
    /// RGB = (85, 85, 85)
    DarkGray,
    /// RGB = (85, 85, 255)
    Blue,
    /// RGB = (85, 255, 85)
    Green,
    /// RGB = (85, 255, 255)
    Cyan,
    /// RGB = (255, 85, 85)
    Red,
    /// RGB = (255, 85, 255)
    Pink,
    /// RGB = (255, 255, 85)
    Yellow,
    /// RGB = (255, 255, 255
    White,
    /// This field is ignored for versions older than 1.16.
    ///
    /// See [`TextColor::custom()`].
    #[cfg(not(feature = "palette"))]
    Custom(FrozenStr),
    #[cfg(feature = "palette")]
    /// This field is ignored for versions older than 1.16.
    Custom(Rgb),
    Reset,
}

#[cfg(not(feature = "palette"))]
impl TextColor {
    pub fn custom<T: Into<FrozenStr>>(color: T) -> TextColor {
        TextColor::Custom(color.into())
    }
}

impl Display for TextColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = String::from(match self {
            TextColor::Black => "black",
            TextColor::DarkBlue => "dark_blue",
            TextColor::DarkGreen => "dark_green",
            TextColor::DarkCyan => "dark_aqua",
            TextColor::DarkRed => "dark_red",
            TextColor::Purple => "dark_purple",
            TextColor::Gold => "gold",
            TextColor::Gray => "gray",
            TextColor::DarkGray => "dark_gray",
            TextColor::Blue => "blue",
            TextColor::Green => "green",
            TextColor::Cyan => "aqua",
            TextColor::Red => "red",
            TextColor::Pink => "light_purple",
            TextColor::Yellow => "yellow",
            TextColor::White => "white",
            TextColor::Custom(color) => {
                #[cfg(feature = "palette")]
                return write!(f, "{}", format!("{color}"));
                #[cfg(not(feature = "palette"))]
                color
            },
            TextColor::Reset => "reset",
        });
        write!(f, "{}", str)
    }
}

impl TryFrom<&str> for TextColor {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "black" => TextColor::Black,
            "dark_blue" => TextColor::DarkBlue,
            "dark_green" => TextColor::DarkGreen,
            "dark_aqua" => TextColor::DarkCyan,
            "dark_red" => TextColor::DarkRed,
            "dark_purple" => TextColor::Purple,
            "gold" => TextColor::Gold,
            "gray" => TextColor::Gray,
            "dark_gray" => TextColor::DarkGray,
            "blue" => TextColor::Blue,
            "green" => TextColor::Green,
            "aqua" => TextColor::Cyan,
            "red" => TextColor::Red,
            "light_purple" => TextColor::Pink,
            "yellow" => TextColor::Yellow,
            "white" => TextColor::White,
            "reset" => TextColor::Reset,
            custom => {
                #[cfg(not(feature = "palette"))]
                if custom.len() != 7 || !custom.starts_with('#') {
                    return Err(());
                } else {
                    for c in custom.chars().skip(1) {
                        if c.is_ascii_hexdigit() {
                            return Err(());
                        }
                    }
                    TextColor::custom(FrozenStr::from(custom))
                }

                #[cfg(feature = "palette")]
                TextColor::Custom(custom.parse().map_err(|_| ())?)
            }
        })
    }
}

#[cfg(feature = "palette")]
mod custom_colors_to_legacy {
    use std::cmp::Ordering;
    use palette::{IntoColor, Lab};
    use crate::{Rgb, TextColor};
    use palette::color_difference::{Ciede2000, EuclideanDistance};

    #[derive(Clone, Copy, PartialOrd, PartialEq)]
    struct Float32Wrapper(f32);

    impl Eq for Float32Wrapper {}
    impl Ord for Float32Wrapper {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.0 == other.0 {
                Ordering::Equal
            } else if self.0 > other.0 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    }

    pub const RGB_COLORS: [(TextColor, (u8, u8, u8)); 16] = [
        (TextColor::Black, (0, 0, 0)),
        (TextColor::DarkBlue, (0, 0, 170)),
        (TextColor::DarkGreen, (0, 170, 0)),
        (TextColor::DarkCyan, (0, 170, 170)),
        (TextColor::DarkRed, (170, 0, 0)),
        (TextColor::Purple, (170, 0, 170)),
        (TextColor::Gold, (255, 170, 0)),
        (TextColor::Gray, (170, 170, 170)),
        (TextColor::DarkGray, (85, 85, 85)),
        (TextColor::Blue, (85, 85, 255)),
        (TextColor::Green, (85, 255, 85)),
        (TextColor::Cyan, (85, 255, 255)),
        (TextColor::Red, (255, 85, 85)),
        (TextColor::Pink, (255, 85, 255)),
        (TextColor::Yellow, (255, 255, 85)),
        (TextColor::White, (255, 255, 255))
    ];

    type ColorCompereFn<T> = fn(Rgb, Rgb) -> T;

    impl TextColor {
        fn into_legacy<T>(self, delta_fn: ColorCompereFn<T>) -> Self where T: Copy, T: Ord {
            if let TextColor::Custom(data) = self {
                *RGB_COLORS.iter()
                    .map(|(color, rgb)| {
                        let delta = delta_fn(data, Rgb::from(*rgb));
                        (color, delta)
                    })
                    .min_by_key(|(_, delta)| *delta)
                    .map_or_else(
                        || unreachable!(), // impossible as long as RGB_COLORS.len() != 0
                        |(color, _)| color
                    )
            } else { self }
        }

        /// Converts [`TextColor::Custom`] to legacy [`TextColor`] values using [`EuclideanDistance`]
        ///
        /// ```rust
        ///  use mc_chat::{Rgb, TextColor};
        ///  assert_eq!(
        ///     TextColor::Custom(Rgb::from((0, 0, 0))).into_legacy_ciede2000(),
        ///     TextColor::Black
        ///  )
        /// ```
        pub fn into_legacy_ciede2000(self) -> Self {
            self.into_legacy(|first, second| {
                let first: Lab = first.0.into_linear().into_color();
                let second: Lab = second.0.into_linear().into_color();

                Float32Wrapper(first.difference(second))
            })
        }

        /// Converts [`TextColor::Custom`] to legacy [`TextColor`] values using [`Ciede2000`]
        ///
        /// ```rust
        ///  use mc_chat::{Rgb, TextColor};
        ///  assert_eq!(
        ///     TextColor::Custom(Rgb::from((255, 255, 255))).into_legacy_euclidean(),
        ///     TextColor::White
        ///  )
        /// ```
        pub fn into_legacy_euclidean(self) -> TextColor {
            self.into_legacy(|first, second| {
                let first: Lab = first.0.into_linear().into_color();
                let second: Lab = second.0.into_linear().into_color();

                Float32Wrapper(first.distance(second))
            })
        }
    }
}

#[cfg(feature = "palette")]
pub use self::custom_colors_to_legacy::*;