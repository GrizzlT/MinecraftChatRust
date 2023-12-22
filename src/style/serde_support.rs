use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

use crate::component::serde_support::{serialize_chat_option, version_option_none, SerializeChat};
use crate::freeze::FrozenStr;
use crate::{Chat, VERSION_1_16};
use serde::de::{self, Unexpected, Visitor};
use serde::ser::{self, SerializeMap, SerializeStruct};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use uuid::Uuid;

use crate::style::{ClickEvent, HoverEvent, Style};
use crate::style::colors::TextColor;

impl Serialize for TextColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&*self.to_string())
    }
}

// TODO: write unit tests
impl<'de> Deserialize<'de> for TextColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input = FrozenStr::deserialize(deserializer)?;
        TextColor::try_from(input.deref()).map_err(|_|
            serde::de::Error::invalid_value(
                Unexpected::Str(&*input),
                &"a 5 digit hex color prefixed by '#'",
            )
        )
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
    NoValueFound(FrozenStr),
}

impl Display for ClickEventDeserializeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClickEventDeserializeErr::WrongKey(str) => write!(f, "{} is not a valid action!", str),
            ClickEventDeserializeErr::NoValueFound(key) => write!(f, "No value found for {}", key),
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
                Err(ClickEventDeserializeErr::NoValueFound(data.action))
            }
        } else if let ClickEventType::String(str) = data.value {
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

#[derive(Serialize)]
struct SerializeEntity<'a> {
    #[serde(skip_serializing_if = "version_option_none")]
    #[serde(serialize_with = "serialize_chat_option")]
    pub name: (i32, &'a Option<Box<Chat>>),
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: &'a Option<FrozenStr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: &'a Option<Uuid>,
}

struct HoverEventSerialize<'a> {
    pub version: i32,
    pub event: &'a HoverEvent,
}

impl<'a> From<(i32, &'a HoverEvent)> for HoverEventSerialize<'a> {
    fn from((version, event): (i32, &'a HoverEvent)) -> Self {
        Self { version, event }
    }
}

impl<'a> Serialize for HoverEventSerialize<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut event = serializer.serialize_struct("hoverEvent", 2)?;
        if let HoverEvent::ShowText(ref text) = self.event {
            event.serialize_field("action", "show_text")?;
            event.serialize_field(
                if self.version < VERSION_1_16 {
                    "value"
                } else {
                    "contents"
                },
                &SerializeChat {
                    kind: (self.version, &text.kind).into(),
                    style: (self.version, &text.style).into(),
                    children: (self.version, &text.children),
                },
            )?;
        } else if self.version < VERSION_1_16 {
            match &self.event {
                HoverEvent::ShowItem(item) => {
                    event.serialize_field("action", "show_item")?;
                    event.serialize_field(
                        "value",
                        &fastsnbt::to_string(&item)
                            .map_err(|_| ser::Error::custom("invalid item"))?,
                    )?;
                }
                HoverEvent::ShowEntity(entity) => {
                    event.serialize_field("action", "show_entity")?;
                    event.serialize_field(
                        "value",
                        &fastsnbt::to_string(&SerializeEntity {
                            name: (self.version, &entity.name),
                            kind: &entity.kind,
                            id: &entity.id,
                        })
                        .map_err(|_| ser::Error::custom("invalid entity data"))?,
                    )?;
                }
                _ => unreachable!("third arm is already matched earlier"),
            }
        } else {
            match &self.event {
                HoverEvent::ShowItem(item) => {
                    event.serialize_field("action", "show_item")?;
                    event.serialize_field("contents", &item)?;
                }
                HoverEvent::ShowEntity(entity) => {
                    event.serialize_field("action", "show_entity")?;
                    event.serialize_field(
                        "contents",
                        &SerializeEntity {
                            name: (self.version, &entity.name),
                            kind: &entity.kind,
                            id: &entity.id,
                        },
                    )?;
                }
                _ => unreachable!("third arm is already matched earlier"),
            }
        }
        event.end()
    }
}

impl<'de> Deserialize<'de> for HoverEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HoverVisitor;

        impl<'de> Visitor<'de> for HoverVisitor {
            type Value = HoverEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("hover event data")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let action: &str;
                let key = map
                    .next_key::<&str>()?
                    .ok_or(de::Error::missing_field("action"))?;
                if key == "action" {
                    action = map.next_value()?;
                    let key = map
                        .next_key::<&str>()?
                        .ok_or(de::Error::missing_field("contents"))?;
                    match (key, action) {
                        ("contents", "show_text") => {
                            Ok(HoverEvent::ShowText(Box::new(map.next_value()?)))
                        }
                        ("contents", "show_item") => Ok(HoverEvent::ShowItem(map.next_value()?)),
                        ("contents", "show_entity") => {
                            Ok(HoverEvent::ShowEntity(map.next_value()?))
                        }
                        ("value", "show_text") => {
                            Ok(HoverEvent::ShowText(Box::new(map.next_value()?)))
                        }
                        ("value", "show_item") => Ok(HoverEvent::ShowItem(
                            fastsnbt::from_str(&map.next_value::<String>()?)
                                .map_err(|e| de::Error::custom(e.to_string()))?,
                        )),
                        ("value", "show_entity") => Ok(HoverEvent::ShowEntity(
                            fastsnbt::from_str(&map.next_value::<String>()?)
                                .map_err(|e| de::Error::custom(e.to_string()))?,
                        )),
                        ("contents", _) => Err(de::Error::invalid_value(
                            Unexpected::Str(key),
                            &"`show_text`, `show_item` or `show_entity`",
                        )),
                        ("value", _) => Err(de::Error::invalid_value(
                            Unexpected::Str(key),
                            &"`show_text`, `show_item` or `show_entity`",
                        )),
                        _ => Err(de::Error::invalid_value(
                            Unexpected::Str(key),
                            &"`contents`, `value`",
                        )),
                    }
                } else if key == "contents" || key == "value" {
                    let content_value = map.next_value::<Value>()?;
                    let _ = map
                        .next_key::<&str>()?
                        .ok_or(de::Error::missing_field("contents"))?;
                    action = map.next_value()?;
                    match (key, action) {
                        ("contents", "show_text") => Ok(HoverEvent::ShowText(Box::new(
                            serde_json::from_value(content_value)
                                .map_err(|_| de::Error::custom("Invalid text component"))?,
                        ))),
                        ("contents", "show_item") => Ok(HoverEvent::ShowItem(
                            serde_json::from_value(content_value)
                                .map_err(|_| de::Error::custom("Invalid itemstack"))?,
                        )),
                        ("contents", "show_entity") => Ok(HoverEvent::ShowEntity(
                            serde_json::from_value(content_value)
                                .map_err(|_| de::Error::custom("Invalid entity"))?,
                        )),
                        ("value", "show_text") => Ok(HoverEvent::ShowText(Box::new(
                            serde_json::from_value(content_value).map_err(|e| {
                                de::Error::custom(format!("Invalid text component: {}", e))
                            })?,
                        ))),
                        ("value", "show_item") => Ok(HoverEvent::ShowItem(
                            fastsnbt::from_str(
                                content_value
                                    .as_str()
                                    .ok_or(de::Error::custom("Expected itemstack sNBT"))?,
                            )
                            .map_err(|e| de::Error::custom(e.to_string()))?,
                        )),
                        ("value", "show_entity") => Ok(HoverEvent::ShowEntity(
                            fastsnbt::from_str(
                                content_value
                                    .as_str()
                                    .ok_or(de::Error::custom("Expected entity sNBT"))?,
                            )
                            .map_err(|e| de::Error::custom(e.to_string()))?,
                        )),
                        _ => Err(de::Error::invalid_value(
                            Unexpected::Str(key),
                            &"`show_text`, `show_item` or `show_entity`",
                        )),
                    }
                } else {
                    Err(de::Error::invalid_value(Unexpected::Str(key), &"`action`"))
                }
            }
        }

        deserializer.deserialize_map(HoverVisitor)
    }
}

pub(crate) struct StyleVersioned<'a> {
    pub version: i32,
    pub style: &'a Style,
}

impl<'a> From<(i32, &'a Style)> for StyleVersioned<'a> {
    fn from((version, style): (i32, &'a Style)) -> Self {
        Self { version, style }
    }
}

impl<'a> Serialize for StyleVersioned<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let version = self.version;
        let style = &self.style;
        let mut map = serializer.serialize_map(None)?;
        if style.bold.is_some() {
            map.serialize_entry("bold", &style.bold)?;
        }
        if style.italic.is_some() {
            map.serialize_entry("italic", &style.italic)?;
        }
        if style.underlined.is_some() {
            map.serialize_entry("underlined", &style.underlined)?;
        }
        if style.strikethrough.is_some() {
            map.serialize_entry("strikethrough", &style.strikethrough)?;
        }
        if style.obfuscated.is_some() {
            map.serialize_entry("obfuscated", &style.obfuscated)?;
        }
        if style.color.is_some() {
            if let Some(TextColor::Custom(_)) = style.color {
                if version >= 713 {
                    map.serialize_entry("color", &style.color)?;
                }
            } else {
                map.serialize_entry("color", &style.color)?;
            }
        }
        if version >= 5 {
            if style.insertion.is_some() {
                map.serialize_entry("insertion", &style.insertion)?;
            }
            if version >= 713 && style.font.is_some() {
                map.serialize_entry("font", &style.font)?;
            }
        }
        if style.click_event.is_some() {
            if let Some(ClickEvent::CopyToClipBoard(_)) = style.click_event {
                if version >= 558 {
                    map.serialize_entry("clickEvent", &style.click_event)?;
                }
            } else {
                map.serialize_entry("clickEvent", &style.click_event)?;
            }
        }
        if let Some(hover_event) = &style.hover_event {
            map.serialize_entry::<_, HoverEventSerialize>(
                "hoverEvent",
                &(version, hover_event).into(),
            )?;
        }

        map.end()
    }
}

#[cfg(test)]
mod tests {
    mod hover_event {
        use crate::{Chat, EntityTooltip, HoverEvent, ItemStack, VERSION_1_16, VERSION_1_8};

        use super::super::HoverEventSerialize;

        #[test]
        pub fn serialize_text() {
            let event = HoverEvent::ShowText(Box::new(Chat::text("Sample text")));
            let serialized_str_pre =
                serde_json::to_string(&HoverEventSerialize::from((VERSION_1_8, &event))).unwrap();
            assert_eq!(
                r#"{"action":"show_text","value":{"text":"Sample text"}}"#,
                serialized_str_pre
            );
            let serialized_str_post =
                serde_json::to_string(&HoverEventSerialize::from((VERSION_1_16, &event))).unwrap();
            assert_eq!(
                r#"{"action":"show_text","contents":{"text":"Sample text"}}"#,
                serialized_str_post
            );
        }

        #[test]
        pub fn serialize_itemstack() {
            let event = HoverEvent::ShowItem(ItemStack::new("diamond", None, Option::<&str>::None));
            let serialized_str_pre =
                serde_json::to_string(&HoverEventSerialize::from((VERSION_1_8, &event))).unwrap();
            assert_eq!(
                r#"{"action":"show_item","value":"{\"id\":\"diamond\"}"}"#,
                serialized_str_pre
            );
            let serialized_str_post =
                serde_json::to_string(&HoverEventSerialize::from((VERSION_1_16, &event))).unwrap();
            assert_eq!(
                r#"{"action":"show_item","contents":{"id":"diamond"}}"#,
                serialized_str_post
            );
        }

        #[test]
        pub fn serialize_entity() {
            let event = HoverEvent::ShowEntity(EntityTooltip::new(
                Some(Chat::text("Sample name")),
                Some("minecraft:pig"),
                None,
            ));
            let serialized_str_pre =
                serde_json::to_string(&HoverEventSerialize::from((VERSION_1_8, &event))).unwrap();
            assert_eq!(
                r#"{"action":"show_entity","value":"{\"name\":{\"text\":\"Sample name\"},\"type\":\"minecraft:pig\"}"}"#,
                serialized_str_pre
            );
            let serialized_str_post =
                serde_json::to_string(&HoverEventSerialize::from((VERSION_1_16, &event))).unwrap();
            assert_eq!(
                r#"{"action":"show_entity","contents":{"name":{"text":"Sample name"},"type":"minecraft:pig"}}"#,
                serialized_str_post
            );
        }

        #[test]
        pub fn deserialize_text() {
            let event_orig = HoverEvent::ShowText(Box::new(Chat::text("Sample text")));

            let serialized_str_pre = r#"{"value":{"text":"Sample text"},"action":"show_text"}"#;
            let serialized_str_post = r#"{"contents":{"text":"Sample text"},"action":"show_text"}"#;
            let event = serde_json::from_str(&serialized_str_pre).unwrap();
            assert_eq!(event_orig, event);
            let event = serde_json::from_str(&serialized_str_post).unwrap();
            assert_eq!(event_orig, event);
        }

        #[test]
        pub fn deserialize_item() {
            let event_orig =
                HoverEvent::ShowItem(ItemStack::new("diamond", Some(30), Option::<&str>::None));

            let serialized_str_pre =
                r#"{"value":"{\"id\":\"diamond\",\"Count\":30}","action":"show_item"}"#;
            let serialized_str_post =
                r#"{"contents":{"id":"diamond","Count":30},"action":"show_item"}"#;
            let event = serde_json::from_str(&serialized_str_pre).unwrap();
            assert_eq!(event_orig, event);
            let event = serde_json::from_str(&serialized_str_post).unwrap();
            assert_eq!(event_orig, event);
        }

        #[test]
        pub fn deserialize_entity() {
            let event_orig = HoverEvent::ShowEntity(EntityTooltip::new(
                Some(Chat::text("Sample name")),
                Some("minecraft:pig"),
                None,
            ));

            let serialized_str_pre = r#"{"action":"show_entity","value":"{\"name\":{\"text\":\"Sample name\"},\"type\":\"minecraft:pig\"}"}"#;
            let serialized_str_post = r#"{"action":"show_entity","contents":{"name":{"text":"Sample name"},"type":"minecraft:pig"}}"#;
            let event = serde_json::from_str(&serialized_str_pre).unwrap();
            assert_eq!(event_orig, event);
            let event = serde_json::from_str(&serialized_str_post).unwrap();
            assert_eq!(event_orig, event);
        }
    }
}
