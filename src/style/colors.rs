

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