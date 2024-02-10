#[cfg(feature = "serde")]
use crate::{component::Chat, freeze::FrozenStr};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(feature = "serde")]
pub(crate) mod serde_support;
mod colors;

pub use colors::*;

/// The style of a [`Chat`] component.
///
/// Style settings are all [`Option`]s to allow
/// child components to reset the
/// inherited style from their parent: a [`Some`]
/// setting **overwrites** the parent's style.
/// The settings are usually modified with the
/// corresponding setters in [`Chat`].
///
/// # Example
/// ```
/// use mc_chat::{Style, TextColor};
///
/// let style = Style::new()
///     .color(TextColor::Green)
///     .bold(true)
///     .obfuscated(true);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct Style {
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub color: Option<TextColor>,
    /// This field is ignored for versions older than 1.8
    pub insertion: Option<FrozenStr>,
    /// This field is ignored for versions older than 1.16
    pub font: Option<FrozenStr>,
    #[cfg_attr(feature = "serde", serde(rename = "clickEvent"))]
    pub click_event: Option<ClickEvent>,
    #[cfg_attr(feature = "serde", serde(rename = "hoverEvent"))]
    pub hover_event: Option<HoverEvent>,
}

impl Style {
    /// Create a new style that inherits everything
    /// from the parent component.
    pub fn new() -> Self {
        Style::default()
    }

    /// Change the text color.
    ///
    /// Because [`TextColor`] implements [`Into<Option<TextColor>>`],
    /// it is very easy to either overwrite or inherit the parent's style.
    ///
    /// # Example
    /// ```
    /// use mc_chat::{Style, TextColor};
    ///
    /// let mut style = Style::new();
    /// // set a color
    /// style.color(TextColor::Green);
    ///
    /// // make the style inherit the parent again
    /// style.color(None);
    /// ```
    pub fn color<I: Into<Option<TextColor>>>(&mut self, color: I) -> &mut Self {
        self.color = color.into();
        self
    }

    pub fn bold(&mut self, bold: bool) -> &mut Self {
        self.bold = Some(bold);
        self
    }

    pub fn italic(&mut self, italic: bool) -> &mut Self {
        self.italic = Some(italic);
        self
    }

    pub fn underlined(&mut self, underlined: bool) -> &mut Self {
        self.underlined = Some(underlined);
        self
    }

    pub fn strikethrough(&mut self, strikethrough: bool) -> &mut Self {
        self.strikethrough = Some(strikethrough);
        self
    }

    pub fn obfuscated(&mut self, obfuscated: bool) -> &mut Self {
        self.obfuscated = Some(obfuscated);
        self
    }

    pub fn font<T: Into<FrozenStr>>(&mut self, font: Option<T>) -> &mut Self {
        self.font = font.map(|font| font.into());
        self
    }

    pub fn insertion<T: Into<FrozenStr>>(&mut self, insertion: Option<T>) -> &mut Self {
        self.insertion = insertion.map(|insertion| insertion.into());
        self
    }

    pub fn click(&mut self, click_event: Option<ClickEvent>) -> &mut Self {
        self.click_event = click_event;
        self
    }

    pub fn hover(&mut self, hover_event: Option<HoverEvent>) -> &mut Self {
        self.hover_event = hover_event;
        self
    }
}


/// A ClickEvent useful in a chat message or book.
/// TODO: Discuss feature gated `open_file` option
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "serde_support::ClickEventData"))]
pub enum ClickEvent {
    OpenUrl(FrozenStr),
    RunCommand(FrozenStr),
    SuggestCommand(FrozenStr),
    ChangePage(u32),
    /// This field is ignored for versions older than 1.15.
    CopyToClipBoard(FrozenStr),
}

impl ClickEvent {
    pub fn url<T: Into<FrozenStr>>(url: T) -> Self {
        Self::OpenUrl(url.into())
    }

    pub fn command<T: Into<FrozenStr>>(cmd: T) -> Self {
        Self::RunCommand(cmd.into())
    }

    pub fn suggest<T: Into<FrozenStr>>(cmd: T) -> Self {
        Self::SuggestCommand(cmd.into())
    }

    pub fn page<T: Into<u32>>(page: T) -> Self {
        Self::ChangePage(page.into())
    }

    pub fn clipboard<T: Into<FrozenStr>>(str: T) -> Self {
        Self::CopyToClipBoard(str.into())
    }
}

/// A HoverEvent useful in a chat message or book.
///
/// # Performance
/// It is highly recommended to provide the data for deserialization
/// with the action first and then the value/contents (based on the version).
/// **Doing otherwise will result in an extra allocation.**
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum HoverEvent {
    ShowText(Box<Chat>),
    ShowItem(ItemStack),
    ShowEntity(EntityTooltip),
}

/// Chat data from an itemstack.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ItemStack {
    pub id: FrozenStr,
    #[cfg_attr(
        feature = "serde",
        serde(
            rename = "Count",
            skip_serializing_if = "Option::is_none",
            default,
            deserialize_with = "optional_serde::deserialize"
        )
    )]
    pub count: Option<i32>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "optional_serde::deserialize")
    )]
    pub tag: Option<FrozenStr>,
}

impl ItemStack {
    pub fn new<I, U>(id: I, count: Option<i32>, tag: Option<U>) -> Self
    where
        I: Into<FrozenStr>,
        U: Into<FrozenStr>,
    {
        Self {
            id: id.into(),
            count,
            tag: tag.map(|t| t.into()),
        }
    }
}

/// Entity tooltip.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct EntityTooltip {
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "optional_serde::deserialize")
    )]
    pub name: Option<Box<Chat>>,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "optional_serde::deserialize")
    )]
    pub kind: Option<FrozenStr>,
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "optional_serde::deserialize")
    )]
    pub id: Option<Uuid>,
}

impl EntityTooltip {
    pub fn new<I>(name: Option<Chat>, kind: Option<I>, id: Option<Uuid>) -> Self
    where
        I: Into<FrozenStr>,
    {
        Self {
            name: name.map(Box::new),
            kind: kind.map(|k| k.into()),
            id,
        }
    }
}

#[cfg(feature = "serde")]
mod optional_serde {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
        deserializer: D,
    ) -> Result<Option<T>, D::Error> {
        Ok(Some(T::deserialize(deserializer)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_itemstack() {
        let itemstack = ItemStack::new("minecraft:clay", Some(10), Some("{other:0}"));
        let str = fastsnbt::to_string(&itemstack).unwrap();
        assert_eq!(
            "{\"id\":\"minecraft:clay\",\"Count\":10,\"tag\":\"{other:0}\"}",
            &str
        );
        let itemstack = ItemStack::new("minecraft:clay", None, Some("{other:2}"));
        let str = fastsnbt::to_string(&itemstack).unwrap();
        assert_eq!("{\"id\":\"minecraft:clay\",\"tag\":\"{other:2}\"}", &str);
    }
}
