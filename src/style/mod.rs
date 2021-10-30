use crate::component::ChatComponent;

#[cfg(feature = "serde-support")]
use serde::Deserialize;
#[cfg(feature = "serde-support")]
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
#[cfg_attr(feature = "serde-support", derive(Deserialize))]
pub struct ComponentStyle {
    #[cfg_attr(
        feature = "serde-support",
        serde(skip, default = "serde_support::default_style_version")
    )]
    pub(crate) version: u32,
    pub(crate) bold: Option<bool>,
    pub(crate) italic: Option<bool>,
    pub(crate) underlined: Option<bool>,
    pub(crate) strikethrough: Option<bool>,
    pub(crate) obfuscated: Option<bool>,
    pub(crate) color: Option<ChatColor>,
    /// This field is ignored for versions older than 1.8
    pub(crate) insertion: Option<String>,
    /// This field is ignored for versions older than 1.16
    pub(crate) font: Option<String>,
    #[cfg_attr(feature = "serde-support", serde(rename = "clickEvent"))]
    pub(crate) click_event: Option<ClickEvent>,
    #[cfg_attr(feature = "serde-support", serde(rename = "hoverEvent"))]
    pub(crate) hover_event: Option<HoverEvent>,
}

impl ComponentStyle {
    pub fn v1_7() -> Self {
        ComponentStyle::with_version(4)
    }

    pub fn v1_8() -> Self {
        ComponentStyle::with_version(47)
    }

    pub fn v1_15() -> Self {
        ComponentStyle::with_version(573)
    }

    pub fn v1_16() -> Self {
        ComponentStyle::with_version(735)
    }

    pub fn with_version(version: u32) -> Self {
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

    pub fn set_color(&mut self, color: Option<ChatColor>) {
        self.color = color;
    }

    pub fn color(mut self, color: Option<ChatColor>) -> Self {
        self.set_color(color);
        self
    }

    pub fn set_color_if_absent(&mut self, color: ChatColor) {
        if self.color.is_none() {
            self.color = Some(color);
        }
    }

    pub fn color_if_absent(mut self, color: ChatColor) -> Self {
        self.set_color_if_absent(color);
        self
    }

    pub fn set_bold(&mut self, bold: bool) {
        self.bold = Some(bold);
    }

    pub fn bold(mut self, bold: bool) -> Self {
        self.set_bold(bold);
        self
    }

    pub fn set_italic(&mut self, italic: bool) {
        self.italic = Some(italic);
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.set_italic(italic);
        self
    }

    pub fn set_underlined(&mut self, underlined: bool) {
        self.underlined = Some(underlined);
    }

    pub fn underlined(mut self, underlined: bool) -> Self {
        self.set_underlined(underlined);
        self
    }

    pub fn set_strikethrough(&mut self, strikethrough: bool) {
        self.strikethrough = Some(strikethrough);
    }

    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.set_strikethrough(strikethrough);
        self
    }

    pub fn set_obfuscated(&mut self, obfuscated: bool) {
        self.obfuscated = Some(obfuscated);
    }

    pub fn obfuscated(mut self, obfuscated: bool) -> Self {
        self.set_obfuscated(obfuscated);
        self
    }

    pub fn set_font<T: Into<String>>(&mut self, font: Option<T>) {
        self.font = font.map(|font| font.into());
    }

    pub fn font<T: Into<String>>(mut self, font: Option<T>) -> Self {
        self.set_font(font);
        self
    }

    pub fn set_insertion<T: Into<String>>(&mut self, insertion: Option<T>) {
        self.insertion = insertion.map(|insertion| insertion.into());
    }

    pub fn insertion<T: Into<String>>(mut self, insertion: Option<T>) -> Self {
        self.set_insertion(insertion);
        self
    }

    pub fn set_click_event(&mut self, click_event: Option<ClickEvent>) {
        self.click_event = click_event;
    }

    pub fn click_event(mut self, click_event: Option<ClickEvent>) -> Self {
        self.set_click_event(click_event);
        self
    }

    pub fn set_hover_event(&mut self, hover_event: Option<HoverEvent>) {
        self.hover_event = hover_event;
    }

    pub fn hover_event(mut self, hover_event: Option<HoverEvent>) -> Self {
        self.set_hover_event(hover_event);
        self
    }

    pub fn get_color(&self) -> Option<&ChatColor> {
        self.color.as_ref()
    }

    pub fn get_bold(&self) -> Option<bool> {
        self.bold
    }

    pub fn get_italic(&self) -> Option<bool> {
        self.italic
    }

    pub fn get_underlined(&self) -> Option<bool> {
        self.underlined
    }

    pub fn get_strikethrough(&self) -> Option<bool> {
        self.strikethrough
    }

    pub fn get_obfuscated(&self) -> Option<bool> {
        self.obfuscated
    }

    pub fn get_font(&self) -> Option<&String> {
        if self.version >= 713 {
            self.font.as_ref()
        } else {
            None
        }
    }

    pub fn get_insertion(&self) -> Option<&String> {
        if self.version >= 5 {
            self.insertion.as_ref()
        } else {
            None
        }
    }

    pub fn get_click_event(&self) -> Option<&ClickEvent> {
        self.click_event.as_ref()
    }

    pub fn get_hover_event(&self) -> Option<&HoverEvent> {
        self.hover_event.as_ref()
    }

    pub fn change_version(&mut self, to: u32) {
        self.version = to;
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
    Custom(String),
    Reset,
}

impl ChatColor {
    pub fn custom<T: Into<String>>(color: T) -> ChatColor {
        ChatColor::Custom(color.into())
    }
}

/// A ClickEvent useful in a chat message or book.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-support", derive(Deserialize))]
#[cfg_attr(
    feature = "serde-support",
    serde(try_from = "serde_support::ClickEventData")
)]
pub enum ClickEvent {
    OpenUrl(String),
    RunCommand(String),
    SuggestCommand(String),
    ChangePage(u32),
    /// This field is ignored for versions older than 1.15.
    CopyToClipBoard(String),
}

impl ClickEvent {
    pub fn url<T: Into<String>>(url: T) -> Self {
        Self::OpenUrl(url.into())
    }

    pub fn run_command<T: Into<String>>(cmd: T) -> Self {
        Self::RunCommand(cmd.into())
    }

    pub fn suggest_command<T: Into<String>>(cmd: T) -> Self {
        Self::SuggestCommand(cmd.into())
    }

    pub fn page<T: Into<u32>>(page: T) -> Self {
        Self::ChangePage(page.into())
    }

    pub fn clipboard<T: Into<String>>(str: T) -> Self {
        Self::CopyToClipBoard(str.into())
    }
}

/// A HoverEvent useful in a chat message or book.
/// ## TODO
/// Change 'value' field to 'contents' when serializing for 1.16+,
/// also add more sophisticated `item` and `entity` data structures
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-support", derive(Deserialize))]
#[cfg_attr(
    feature = "serde-support",
    serde(try_from = "serde_support::HoverEventData")
)]
pub enum HoverEvent {
    ShowText(Box<ChatComponent>),
    ShowItem(String),
    ShowEntity(String),
}
