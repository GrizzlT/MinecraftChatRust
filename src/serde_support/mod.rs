use erased_serde::serialize_trait_object;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::{ChatColor, ClickEvent, ComponentStyle, HoverEvent};

serialize_trait_object!(crate::Component);

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
            ClickEvent::CopyToClipBoard(value) => {
                item.serialize_field("action", "copy_to_clipboard")?;
                item.serialize_field("value", value)?;
            }
        }
        item.end()
    }
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

impl Serialize for ComponentStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            ComponentStyle::V1_7(style) => style.serialize(serializer),
            ComponentStyle::V1_8(style) => style.serialize(serializer),
            ComponentStyle::V1_16(style) => style.serialize(serializer),
        }
    }
}