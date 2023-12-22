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
    use palette::{IntoColor, Lab};
    use crate::{Rgb, TextColor};
    use palette::color_difference::{Ciede2000, EuclideanDistance};

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

        fn into_legacy<T: PartialOrd>(self, delta_fn: ColorCompereFn<T>) -> Self {
            match self {
                TextColor::Custom(data) => {
                    let mut min: Option<(TextColor, T)> = None;
                    for (color, rgb) in RGB_COLORS {
                        let delta = delta_fn(data, Rgb::from(rgb));
                        if let Some((_, value)) = &min {
                            if value > &delta { min = Some((color, delta)) }
                        } else {
                            min = Some((color, delta))
                        }
                    }

                    match min {
                        Some((color, _)) => color,
                        None => unreachable!()
                    }
                }
                color => color
            }
        }
        pub fn to_legacy_ciede2000(self) -> Self {
            self.into_legacy(|first, second| {
                let first: Lab = first.0.into_linear().into_color();
                let second: Lab = second.0.into_linear().into_color();

                first.difference(second)
            })
        }
    }
}

#[cfg(feature = "palette")]
pub use self::custom_colors_to_legacy::*;

#[cfg(feature = "palette")]
#[cfg(test)]
mod tests {
    use crate::{Rgb, TextColor};

    #[test]
    fn test_ciede200_conversion() {
        assert_eq!(TextColor::Custom(Rgb::from((0, 0, 0))).to_legacy_ciede2000(), TextColor::Black)
    }
}