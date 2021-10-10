#![allow(dead_code)]

pub use component::Component;
pub use component::ComponentStyle;

use serde::{Serialize, Deserialize, Serializer};
use serde::ser::SerializeStruct;

pub mod text;
#[doc(hidden)]
pub mod component;

mod tests;

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
    Custom(String),
    Reset
}

impl Serialize for ChatColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(match self {
            ChatColor::Black => "black",
            ChatColor::DarkBlue => "dark_blue",
            ChatColor::DarkGreen => "dark_green",
            ChatColor::DarkCyan => "dark_aqua",
            ChatColor::DarkRed => "dark-red",
            ChatColor::Purple => "dark_purple",
            ChatColor::Gold => "gold",
            ChatColor::Gray => "gray",
            ChatColor::DarkGray => "dark_gray",
            ChatColor::Blue => "blue",
            ChatColor::Green => "green",
            ChatColor::Cyan => "aqua",
            ChatColor::Red => "red",
            ChatColor::Pink => "light_purple",
            ChatColor::Yellow => "yellow",
            ChatColor::White => "white",
            ChatColor::Custom(color) => color,
            ChatColor::Reset => "reset"
        })
    }
}

#[derive(Deserialize)]
pub enum ClickEvent {
    OpenUrl(String),
    RunCommand(String),
    SuggestCommand(String),
    ChangePage(u32),
    CopyToClipBoard
}

impl Serialize for ClickEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut item = serializer.serialize_struct("clickEvent", 2)?;
        match self {
            ClickEvent::OpenUrl(url) => {
                item.serialize_field("action", "open_url")?;
                item.serialize_field("value", url)?;
            }
            ClickEvent::RunCommand(cmd) => {
                item.serialize_field("action", "run_command")?;
                item.serialize_field("value", cmd)?;
            }
            ClickEvent::SuggestCommand(cmd) => {
                item.serialize_field("action", "suggest_command")?;
                item.serialize_field("value", cmd)?;
            }
            ClickEvent::ChangePage(page) => {
                item.serialize_field("action", "change_page")?;
                item.serialize_field("value", page)?;
            }
            ClickEvent::CopyToClipBoard => {
                item.serialize_field("action", "copy_to_clipboard")?;
            }
        }
        item.end()
    }
}

pub enum HoverEvent {
    ShowText(Box<dyn Component>),
    ShowItem(String),
    ShowEntity(String)
}

impl Serialize for HoverEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut event = serializer.serialize_struct("hoverEvent", 2)?;
        match self {
            HoverEvent::ShowText(text) => {
                event.serialize_field("action", "show_text")?;
                event.serialize_field("value", text)?;
            }
            HoverEvent::ShowItem(item) => {
                event.serialize_field("action", "show_item")?;
                event.serialize_field("value", item)?;
            }
            HoverEvent::ShowEntity(entity) => {
                event.serialize_field("action", "show_entity")?;
                event.serialize_field("value", entity)?;
            }
        }
        event.end()
    }
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
        self.get_style_mut().bold = if bold { Some(()) } else { None };
        self
    }

    fn italic(mut self, italic: bool) -> Self {
        self.get_style_mut().italic = if italic { Some(()) } else { None };
        self
    }

    fn underlined(mut self, underlined: bool) -> Self {
        self.get_style_mut().underlined = if underlined { Some(()) } else { None };
        self
    }

    fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.get_style_mut().strikethrough = if strikethrough { Some(()) } else { None };
        self
    }

    fn obfuscated(mut self, obfuscated: bool) -> Self {
        self.get_style_mut().obfuscated = if obfuscated { Some(()) } else { None };
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
