//!
//! This crate provides Rust objects that map to the minecraft
//! protocol raw JSON message format used for chat messages, books, titles...
//!
//! Serialization and Deserialization is planned to be implemented for both legacy and json strings.
//!
//! Currently this is still a work in progress so please check out our [github](https://github.com/GrizzlT/MinecraftChatRust) and
//! feel free to contribute.

pub use component::Component;
pub use component::ComponentStyle;
pub use text::TextComponent;
pub use text::TranslatableComponent;
use crate::component::ComponentStyleEditable;

#[macro_use]
mod macros;

mod text;
mod component;
mod tests;

#[cfg(feature = "serde-support")]
mod serde_support;

/// The different colors a [`Component`] can have.
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
    /// # Warning
    /// This field was introduced in 1.16 and must be a valid 6-digit hexadecimal value prefixed by a `#`.
    ///
    /// Implementations of serializers for older versions should ignore this field at all times.
    Custom(String),
    Reset
}

/// A ClickEvent useful in a chat message or book.
pub enum ClickEvent {
    OpenUrl(String),
    RunCommand(String),
    SuggestCommand(String),
    ChangePage(u32),
    /// # Warning
    /// This field was introduced in 1.15.
    ///
    /// Implementations of serializers for older versions should ignore this field at all times.
    CopyToClipBoard(String)
}

/// A HoverEvent useful in a chat message or book.
pub enum HoverEvent {
    ShowText(Box<dyn Component>),
    ShowItem(String),
    ShowEntity(String)
}

/// Defines the ability of a component to change their style.
pub trait DecorateComponent {
    fn color(self, color: Option<ChatColor>) -> Self;

    fn color_if_absent(self, color: ChatColor) -> Self;

    fn bold(self, bold: bool) -> Self;

    fn italic(self, italic: bool) -> Self;

    fn underlined(self, underlined: bool) -> Self;

    fn strikethrough(self, strikethrough: bool) -> Self;

    fn obfuscated(self, obfuscated: bool) -> Self;

    fn font(self, font: Option<String>) -> Self;

    fn insertion(self, insertion: Option<String>) -> Self;

    fn click_event(self, click_event: Option<ClickEvent>) -> Self;

     fn hover_event(self, hover_event: Option<HoverEvent>) -> Self;

    /// Tries to assign all fields from the specified style to this object.
    ///
    /// Implementations should indicate which fields are assigned and which fields are not.
    fn apply_style(self, style: &ComponentStyle) -> Self;

    fn reset_style(self) -> Self;
}

impl<T: Component> DecorateComponent for T {
    fn color(mut self, color: Option<ChatColor>) -> Self {
        self.get_style_mut().color(color);
        self
    }

    fn color_if_absent(mut self, color: ChatColor) -> Self {
        if self.get_style().color_absent() {
            self.get_style_mut().color(Some(color));
        }
        self
    }

    fn bold(mut self, bold: bool) -> Self {
        self.get_style_mut().bold(bold);
        self
    }

    fn italic(mut self, italic: bool) -> Self {
        self.get_style_mut().italic(italic);
        self
    }

    fn underlined(mut self, underlined: bool) -> Self {
        self.get_style_mut().underlined(underlined);
        self
    }

    fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.get_style_mut().strikethrough(strikethrough);
        self
    }

    fn obfuscated(mut self, obfuscated: bool) -> Self {
        self.get_style_mut().obfuscated(obfuscated);
        self
    }

    fn font(mut self, font: Option<String>) -> Self {
        self.get_style_mut().font(font);
        self
    }

    fn insertion(mut self, insertion: Option<String>) -> Self {
        self.get_style_mut().insertion(insertion);
        self
    }

    fn click_event(mut self, click_event: Option<ClickEvent>) -> Self {
        self.get_style_mut().click_event(click_event);
        self
    }

    fn hover_event(mut self, hover_event: Option<HoverEvent>) -> Self {
        self.get_style_mut().hover_event(hover_event);
        self
    }

    fn apply_style(mut self, style: &ComponentStyle) -> Self {
        self.get_style_mut().apply_style(style);
        self
    }

    fn reset_style(mut self) -> Self {
        self.get_style_mut().reset();
        self
    }
}
