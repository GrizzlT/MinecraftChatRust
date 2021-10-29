use serde::{Serialize, Serializer};
use serde::ser::{SerializeMap, SerializeStruct};
use crate::style::{ChatColor, ClickEvent, ComponentStyle, HoverEvent};

impl Serialize for ChatColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(match self {
            ChatColor::Black => "black",
            ChatColor::DarkBlue => "dark_blue",
            ChatColor::DarkGreen => "dark_green",
            ChatColor::DarkCyan => "dark_aqua",
            ChatColor::DarkRed => "dark_red",
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
            ChatColor::Reset => "reset",
        })
    }
}

impl Serialize for ClickEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
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
                item.serialize_field("value", &page.to_string())?;
            }
            ClickEvent::CopyToClipBoard(value) => {
                item.serialize_field("action", "copy_to_clipboard")?;
                item.serialize_field("value", value)?;
            }
        }
        item.end()
    }
}

/// TODO: change serialization to `contents` instead of `value`
impl Serialize for HoverEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
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

impl Serialize for ComponentStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        if self.bold.is_some() {
            map.serialize_entry("bold", &self.bold)?;
        }
        if self.italic.is_some() {
            map.serialize_entry("italic", &self.italic)?;
        }
        if self.underlined.is_some() {
            map.serialize_entry("underlined", &self.underlined)?;
        }
        if self.strikethrough.is_some() {
            map.serialize_entry("strikethrough", &self.strikethrough)?;
        }
        if self.obfuscated.is_some() {
            map.serialize_entry("obfuscated", &self.obfuscated)?;
        }
        if self.color.is_some() {
            if let Some(ChatColor::Custom(_)) = self.color {
                if self.version >= 713 {
                    map.serialize_entry("color", &self.color)?;
                }
            } else {
                map.serialize_entry("color", &self.color)?;
            }
        }
        if self.version >= 5 {
            if self.insertion.is_some() {
                map.serialize_entry("insertion", &self.insertion)?;
            }
            if self.version >= 713 {
                if self.font.is_some() {
                    map.serialize_entry("font", &self.font)?;
                }
            }
        }
        if self.click_event.is_some() {
            if let Some(ClickEvent::CopyToClipBoard(_)) = self.click_event {
                if self.version >= 558 {
                    map.serialize_entry("clickEvent", &self.click_event)?;
                }
            } else {
                map.serialize_entry("clickEvent", &self.click_event)?;
            }
        }
        if self.hover_event.is_some() {
            map.serialize_entry("hoverEvent", &self.hover_event)?;
        }

        map.end()
    }
}