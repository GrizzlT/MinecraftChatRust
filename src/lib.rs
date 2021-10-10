#![allow(dead_code)]

pub use component::Component;
pub use component::ComponentStyle;

use serde::{Serialize, Deserialize};

pub mod text;
#[doc(hidden)]
pub mod component;

mod tests;

#[derive(Clone, Serialize, Deserialize)]
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
    Custom(String),
    Reset
}

#[derive(Serialize, Deserialize)]
pub enum ClickEvent {
    OpenUrl(String),
    RunCommand(String),
    SuggestCommand(String),
    ChangePage(u32),
    CopyToClipBoard
}

#[derive(Serialize)]
pub enum HoverEvent {
    ShowText(Box<dyn Component>),
    ShowItem(String),
    ShowEntity(String)
}

pub trait DecorateComponent {
    fn color(self, color: Option<ChatColor>) -> Self;

    fn color_if_absent(self, color: ChatColor) -> Self;

    fn bold(self, bold: bool) -> Self;

    fn italic(self, italic: bool) -> Self;

    fn underlined(self, underlined: bool) -> Self;

    fn strikethrough(self, strikethrough: bool) -> Self;

    fn obfuscated(self, obfuscated: bool) -> Self;

    fn insertion(self, insertion: Option<String>) -> Self;

    fn click_event(self, click_event: Option<ClickEvent>) -> Self;

     fn hover_event(self, hover_event: Option<HoverEvent>) -> Self;

    fn apply_style(self, style: &ComponentStyle) -> Self;

    fn merge_style(self, style: &ComponentStyle) -> Self;

    fn reset_style(self) -> Self;
}

impl<T: Component> DecorateComponent for T {
    fn color(mut self, color: Option<ChatColor>) -> Self {
        self.get_style_mut().color = color;
        self
    }

    fn color_if_absent(mut self, color: ChatColor) -> Self {
        if self.get_style().color.is_none() {
            self.get_style_mut().color = Some(color);
        }
        self
    }

    fn bold(mut self, bold: bool) -> Self {
        self.get_style_mut().bold = bold;
        self
    }

    fn italic(mut self, italic: bool) -> Self {
        self.get_style_mut().italic = italic;
        self
    }

    fn underlined(mut self, underlined: bool) -> Self {
        self.get_style_mut().underlined = underlined;
        self
    }

    fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.get_style_mut().strikethrough = strikethrough;
        self
    }

    fn obfuscated(mut self, obfuscated: bool) -> Self {
        self.get_style_mut().obfuscated = obfuscated;
        self
    }

    fn insertion(mut self, insertion: Option<String>) -> Self {
        self.get_style_mut().insertion = insertion;
        self
    }

    fn click_event(mut self, click_event: Option<ClickEvent>) -> Self {
        self.get_style_mut().click_event = click_event;
        self
    }

    fn hover_event(mut self, hover_event: Option<HoverEvent>) -> Self {
        self.get_style_mut().hover_event = hover_event;
        self
    }

    fn apply_style(mut self, style: &ComponentStyle) -> Self {
        self.get_style_mut().apply_style(style);
        self
    }

    fn merge_style(mut self, style: &ComponentStyle) -> Self {
        self.get_style_mut().merge_style(style);
        self
    }

    fn reset_style(mut self) -> Self {
        self.get_style_mut().reset();
        self
    }
}
