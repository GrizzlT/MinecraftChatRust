use crate::component::ChatComponent;

#[cfg(feature = "serde-support")]
mod serde_support;

/// The style of a [`ChatComponent`]
#[derive(Clone)]
pub struct ComponentStyle {
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
    pub(crate) click_event: Option<ClickEvent>,
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
            hover_event: None
        }
    }

    pub fn set_color(&mut self, color: Option<ChatColor>) {
        self.color = color;
    }

    pub fn color(mut self, color: Option<ChatColor>) -> Self {
        self.color = color;
        self
    }

    pub fn set_color_if_absent(&mut self, color: ChatColor) {
        if self.color.is_none() {
            self.color = Some(color);
        }
    }

    pub fn color_if_absent(mut self, color: ChatColor) -> Self {
        if self.color.is_none() {
            self.color = Some(color);
        }
        self
    }

    pub fn set_bold(&mut self, bold: bool) {
        self.bold = Some(bold);
    }

    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = Some(bold);
        self
    }

    pub fn set_italic(&mut self, italic: bool) {
        self.italic = Some(italic);
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = Some(italic);
        self
    }

    pub fn set_underlined(&mut self, underlined: bool) {
        self.underlined = Some(underlined);
    }

    pub fn underlined(mut self, underlined: bool) -> Self {
        self.underlined = Some(underlined);
        self
    }

    pub fn set_strikethrough(&mut self, strikethrough: bool) {
        self.strikethrough = Some(strikethrough);
    }

    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = Some(strikethrough);
        self
    }

    pub fn set_obfuscated(&mut self, obfuscated: bool) {
        self.obfuscated = Some(obfuscated);
    }

    pub fn obfuscated(mut self, obfuscated: bool) -> Self {
        self.obfuscated = Some(obfuscated);
        self
    }

    pub fn set_font(&mut self, font: Option<String>) {
        self.font = font;
    }

    pub fn font(mut self, font: Option<String>) -> Self {
        self.font = font;
        self
    }

    pub fn set_insertion(&mut self, insertion: Option<String>) {
        self.insertion = insertion;
    }

    pub fn insertion(mut self, insertion: Option<String>) -> Self {
        self.insertion = insertion;
        self
    }

    pub fn set_click_event(&mut self, click_event: Option<ClickEvent>) {
        self.click_event = click_event;
    }

    pub fn click_event(mut self, click_event: Option<ClickEvent>) -> Self {
        self.click_event = click_event;
        self
    }

    pub fn set_hover_event(&mut self, hover_event: Option<HoverEvent>) {
        self.hover_event = hover_event;
    }

    pub fn hover_event(mut self, hover_event: Option<HoverEvent>) -> Self {
        self.hover_event = hover_event;
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
#[derive(Clone)]
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
    Custom(String),
    Reset,
}

/// A ClickEvent useful in a chat message or book.
#[derive(Clone)]
pub enum ClickEvent {
    OpenUrl(String),
    RunCommand(String),
    SuggestCommand(String),
    ChangePage(u32),
    /// This field is ignored for versions older than 1.15.
    CopyToClipBoard(String),
}

/// A HoverEvent useful in a chat message or book.
/// ## TODO
/// Change 'value' field to 'contents' when serializing for 1.16+
#[derive(Clone)]
pub enum HoverEvent {
    ShowText(Box<ChatComponent>),
    ShowItem(String),
    ShowEntity(String),
}