use crate::{component::Chat, freeze::FrozenStr};

#[cfg(feature = "serde")]
use serde::Deserialize;
use uuid::Uuid;
#[cfg(feature = "serde")]
mod serde_support;

/// The style of a [`ChatComponent`]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct Style {
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub color: Option<ChatColor>,
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
    pub fn new() -> Self {
        Style::default()
    }

    pub fn and_color(mut self, color: Option<ChatColor>) -> Self {
        self.color = color;
        self
    }

    pub fn color(mut self, color: ChatColor) -> Self {
        self.color = Some(color);
        self
    }

    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = Some(bold);
        self
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = Some(italic);
        self
    }

    pub fn underlined(mut self, underlined: bool) -> Self {
        self.underlined = Some(underlined);
        self
    }

    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = Some(strikethrough);
        self
    }

    pub fn obfuscated(mut self, obfuscated: bool) -> Self {
        self.obfuscated = Some(obfuscated);
        self
    }

    pub fn font<T: Into<FrozenStr>>(mut self, font: Option<T>) -> Self {
        self.font = font.map(|font| font.into());
        self
    }

    pub fn insertion<T: Into<FrozenStr>>(mut self, insertion: Option<T>) -> Self {
        self.insertion = insertion.map(|insertion| insertion.into());
        self
    }

    pub fn click(mut self, click_event: Option<ClickEvent>) -> Self {
        self.click_event = click_event;
        self
    }

    pub fn hover(mut self, hover_event: Option<HoverEvent>) -> Self {
        self.hover_event = hover_event;
        self
    }
}

/// The different colors a [`ChatComponent`] can have.
/// ## TODO
/// Automatically find nearest value when serializing [`ChatColor::Custom`] for older versions
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ChatColor {
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
    /// See [`ChatColor::custom()`].
    Custom(FrozenStr),
    Reset,
}

impl ChatColor {
    pub fn custom<T: Into<FrozenStr>>(color: T) -> ChatColor {
        ChatColor::Custom(color.into())
    }
}

/// A ClickEvent useful in a chat message or book.
/// TODO: feature gated `open_file` option
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
/// ## TODO
/// Change 'value' field to 'contents' when serializing for 1.16+,
/// also add more sophisticated `item` and `entity` data structures
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "serde_support::HoverEventData"))]
pub enum HoverEvent {
    ShowText(Box<Chat>),
    ShowItem(ItemStack),
    ShowEntity(FrozenStr),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ItemStack {
    pub id: FrozenStr,
    pub count: i32,
    pub tag: FrozenStr,
}

impl ItemStack {
    pub fn new<I, U>(id: I, count: i32, tag: U) -> Self
    where
        I: Into<FrozenStr>,
        U: Into<FrozenStr>,
    {
        Self {
            id: id.into(),
            count,
            tag: tag.into(),
        }
    }
}

pub struct EntityTooltip {
    pub name: Box<Chat>,
    pub kind: FrozenStr,
    pub id: Uuid,
}

impl EntityTooltip {
    pub fn new<I>(name: Chat, kind: I, id: Uuid) -> Self
    where
        I: Into<FrozenStr>,
    {
        Self {
            name: Box::new(name),
            kind: kind.into(),
            id,
        }
    }
}
