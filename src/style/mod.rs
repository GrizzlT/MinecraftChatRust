use crate::{component::ChatComponent, freeze::FreezeStr};

#[cfg(feature = "serde")]
use serde::Deserialize;
#[cfg(feature = "serde")]
mod serde_support;

/// The version number of the Minecraft protocol for 1.7
pub const VERSION_1_7: u32 = 4;
/// The version number of the Minecraft protocol for 1.8
pub const VERSION_1_8: u32 = 47;
/// The version number of the Minecraft protocol for 1.15
pub const VERSION_1_15: u32 = 573;
/// The version number of the Minecraft protocol for 1.16
pub const VERSION_1_16: u32 = 735;

/// The style of a [`ChatComponent`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct ComponentStyle {
    #[cfg_attr(
        feature = "serde",
        serde(skip, default = "serde_support::default_style_version")
    )]
    pub version: u32,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub color: Option<ChatColor>,
    /// This field is ignored for versions older than 1.8
    pub insertion: Option<FreezeStr>,
    /// This field is ignored for versions older than 1.16
    pub font: Option<FreezeStr>,
    #[cfg_attr(feature = "serde", serde(rename = "clickEvent"))]
    pub click_event: Option<ClickEvent>,
    #[cfg_attr(feature = "serde", serde(rename = "hoverEvent"))]
    pub hover_event: Option<HoverEvent>,
}

impl ComponentStyle {
    pub fn v1_7() -> Self {
        ComponentStyle::new(4)
    }

    pub fn v1_8() -> Self {
        ComponentStyle::new(47)
    }

    pub fn v1_15() -> Self {
        ComponentStyle::new(573)
    }

    pub fn v1_16() -> Self {
        ComponentStyle::new(735)
    }

    pub fn new(version: u32) -> Self {
        ComponentStyle {
            version,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            color: None,
            insertion: None,
            font: None,
            click_event: None,
            hover_event: None,
        }
    }

    pub fn and_color(mut self, color: Option<ChatColor>) -> Self {
        self.color = color;
        self
    }

    pub fn color(mut self, color: ChatColor) -> Self {
        if self.color.is_none() {
            self.color = Some(color);
        }
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

    pub fn font<T: Into<FreezeStr>>(mut self, font: Option<T>) -> Self {
        self.font = font.map(|font| font.into());
        self
    }

    pub fn insertion<T: Into<FreezeStr>>(mut self, insertion: Option<T>) -> Self {
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

    pub fn version(mut self, to: u32) -> Self {
        self.version = to;
        self
    }

    /// Resets all fields to default (being [`None`]).
    pub fn reset(&mut self) {
        self.bold = None;
        self.italic = None;
        self.underlined = None;
        self.strikethrough = None;
        self.obfuscated = None;
        self.color = None;
        self.insertion = None;
        self.font = None;
        self.click_event = None;
        self.hover_event = None;
    }

    pub fn freeze(&mut self) {
        if let Some(color) = &mut self.color {
            color.freeze();
        }
        if let Some(insertion) = &mut self.insertion {
            insertion.freeze();
        }
        if let Some(font) = &mut self.font {
            font.freeze();
        }
        if let Some(event) = &mut self.click_event {
            event.freeze();
        }
        if let Some(event) = &mut self.hover_event {
            event.freeze();
        }
    }
}

/// The different colors a [`ChatComponent`] can have.
/// ## TODO
/// Automatically find nearest value when serializing [`ChatColor::Custom`] for older versions
#[derive(Clone, Debug)]
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
    Custom(FreezeStr),
    Reset,
}

impl ChatColor {
    pub fn custom<T: Into<FreezeStr>>(color: T) -> ChatColor {
        ChatColor::Custom(color.into())
    }

    pub fn freeze(&mut self) {
        match self {
            Self::Custom(str) => str.freeze(),
            _ => {}
        }
    }
}

/// A ClickEvent useful in a chat message or book.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "serde_support::ClickEventData"))]
pub enum ClickEvent {
    OpenUrl(FreezeStr),
    RunCommand(FreezeStr),
    SuggestCommand(FreezeStr),
    ChangePage(u32),
    /// This field is ignored for versions older than 1.15.
    CopyToClipBoard(FreezeStr),
}

impl ClickEvent {
    pub fn url<T: Into<FreezeStr>>(url: T) -> Self {
        Self::OpenUrl(url.into())
    }

    pub fn command<T: Into<FreezeStr>>(cmd: T) -> Self {
        Self::RunCommand(cmd.into())
    }

    pub fn suggest<T: Into<FreezeStr>>(cmd: T) -> Self {
        Self::SuggestCommand(cmd.into())
    }

    pub fn page<T: Into<u32>>(page: T) -> Self {
        Self::ChangePage(page.into())
    }

    pub fn clipboard<T: Into<FreezeStr>>(str: T) -> Self {
        Self::CopyToClipBoard(str.into())
    }

    pub fn freeze(&mut self) {
        match self {
            Self::OpenUrl(str) => str.freeze(),
            Self::RunCommand(str) => str.freeze(),
            Self::SuggestCommand(str) => str.freeze(),
            Self::CopyToClipBoard(str) => str.freeze(),
            _ => {}
        }
    }
}

/// A HoverEvent useful in a chat message or book.
/// ## TODO
/// Change 'value' field to 'contents' when serializing for 1.16+,
/// also add more sophisticated `item` and `entity` data structures
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "serde_support::HoverEventData"))]
pub enum HoverEvent {
    ShowText(Box<ChatComponent>),
    ShowItem(FreezeStr),
    ShowEntity(FreezeStr),
}

impl HoverEvent {
    pub fn freeze(&mut self) {
        match self {
            HoverEvent::ShowText(text) => text.freeze(),
            HoverEvent::ShowItem(str) => str.freeze(),
            HoverEvent::ShowEntity(str) => str.freeze(),
        }
    }
}
