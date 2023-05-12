use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

use crate::component::Chat;
use crate::freeze::FrozenStr;
use serde::ser::{SerializeMap, SerializeStruct};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::style::{ChatColor, ClickEvent, Style, HoverEvent, VERSION_1_16};

impl Serialize for ChatColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
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

impl<'de> Deserialize<'de> for ChatColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input = FrozenStr::deserialize(deserializer)?;
        Ok(match input.deref() {
            "black" => ChatColor::Black,
            "dark_blue" => ChatColor::DarkBlue,
            "dark_green" => ChatColor::DarkGreen,
            "dark_aqua" => ChatColor::DarkCyan,
            "dark_red" => ChatColor::DarkRed,
            "dark_purple" => ChatColor::Purple,
            "gold" => ChatColor::Gold,
            "gray" => ChatColor::Gray,
            "dark_gray" => ChatColor::DarkGray,
            "blue" => ChatColor::Blue,
            "green" => ChatColor::Green,
            "aqua" => ChatColor::Cyan,
            "red" => ChatColor::Red,
            "light_purple" => ChatColor::Pink,
            "yellow" => ChatColor::Yellow,
            "white" => ChatColor::White,
            "reset" => ChatColor::Reset,
            _ => ChatColor::Custom(input),
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

#[derive(Deserialize)]
#[serde(untagged)]
enum ClickEventType {
    String(FrozenStr),
    U32(u32),
}

#[derive(Deserialize)]
pub(crate) struct ClickEventData {
    action: FrozenStr,
    value: ClickEventType,
}

pub enum ClickEventDeserializeErr {
    WrongKey(FrozenStr),
    NoValuFound(FrozenStr),
}

impl Display for ClickEventDeserializeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClickEventDeserializeErr::WrongKey(str) => write!(f, "{} is not a valid action!", str),
            ClickEventDeserializeErr::NoValuFound(key) => write!(f, "No value found for {}", key),
        }
    }
}

impl TryFrom<ClickEventData> for ClickEvent {
    type Error = ClickEventDeserializeErr;

    fn try_from(data: ClickEventData) -> Result<Self, Self::Error> {
        if data.action.deref() == "change_page" {
            if let ClickEventType::U32(value) = data.value {
                Ok(ClickEvent::ChangePage(value))
            } else {
                Err(ClickEventDeserializeErr::NoValuFound(data.action))
            }
        } else {
            if let ClickEventType::String(str) = data.value {
                match data.action.deref() {
                    "open_url" => Ok(ClickEvent::OpenUrl(str)),
                    "run_command" => Ok(ClickEvent::RunCommand(str)),
                    "suggest_command" => Ok(ClickEvent::SuggestCommand(str)),
                    "copy_to_clipboard" => Ok(ClickEvent::CopyToClipBoard(str)),
                    _ => Err(ClickEventDeserializeErr::WrongKey(str)),
                }
            } else {
                Err(ClickEventDeserializeErr::WrongKey(data.action))
            }
        }
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

#[derive(Deserialize)]
#[serde(untagged)]
enum HoverEventType {
    String(FrozenStr),
    Chat(Chat),
}

#[derive(Deserialize)]
pub(crate) struct HoverEventData {
    action: FrozenStr,
    value: HoverEventType,
}

pub enum HoverEventDeserializeErr {
    WrongKey(FrozenStr),
    NoValueFound(FrozenStr),
}

impl Display for HoverEventDeserializeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HoverEventDeserializeErr::WrongKey(str) => write!(f, "{} is not a valid action!", str),
            HoverEventDeserializeErr::NoValueFound(key) => {
                write!(f, "Couldn't find appropriate value for {}", key)
            }
        }
    }
}

impl TryFrom<HoverEventData> for HoverEvent {
    type Error = HoverEventDeserializeErr;

    fn try_from(data: HoverEventData) -> Result<Self, Self::Error> {
        if data.action.deref() == "show_text" {
            if let HoverEventType::Chat(component) = data.value {
                Ok(HoverEvent::ShowText(Box::new(component)))
            } else {
                Err(HoverEventDeserializeErr::NoValueFound(data.action))
            }
        } else {
            if let HoverEventType::String(str) = data.value {
                match data.action.deref() {
                    "show_item" => Ok(HoverEvent::ShowItem(str)),
                    "show_entity" => Ok(HoverEvent::ShowEntity(str)),
                    _ => Err(HoverEventDeserializeErr::WrongKey(str)),
                }
            } else {
                Err(HoverEventDeserializeErr::WrongKey(data.action))
            }
        }
    }
}

impl Serialize for Style {
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

#[inline]
pub(crate) fn default_style_version() -> u32 {
    VERSION_1_16
}
