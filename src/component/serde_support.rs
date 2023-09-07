use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use crate::{ComponentKind, TextComponent, ScoreComponent, KeybindComponent};
use crate::freeze::FrozenStr;
use crate::style::serde_support::StyleVersioned;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize, Serializer};

use crate::style::Style;

use super::Chat;

#[derive(Serialize, Deserialize)]
pub(crate) struct SerializeScore {
    score: SerializeScoreInner,
}

impl From<ScoreComponent> for SerializeScore {
    fn from(value: ScoreComponent) -> Self {
        SerializeScore { score: SerializeScoreInner { name: value.name, objective: value.objective, value: value.value } }
    }
}

impl From<SerializeScore> for ScoreComponent {
    fn from(value: SerializeScore) -> Self {
        ScoreComponent { name: value.score.name, objective: value.score.objective, value: value.score.value }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SerializeScoreInner {
    pub name: FrozenStr,
    pub objective: FrozenStr,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub value: Option<FrozenStr>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FakeChatComponent {
    #[serde(flatten)]
    kind: ComponentKind,
    #[serde(flatten)]
    style: Style,
    #[serde(rename = "extra", default)]
    children: Vec<Chat>,
}

#[doc(hidden)]
impl From<FakeChatComponent> for Chat {
    fn from(component: FakeChatComponent) -> Self {
        Chat {
            kind: component.kind,
            style: component.style,
            children: component.children,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum ChatComponentType {
    Primitive(String),
    Array(Vec<Chat>),
    Object(FakeChatComponent),
}

pub enum ChatComponentDeserializeErr {
    EmptyArray,
}

impl Display for ChatComponentDeserializeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Empty arrays are invalid for chat components!")
    }
}

impl TryFrom<ChatComponentType> for Chat {
    type Error = ChatComponentDeserializeErr;

    fn try_from(value: ChatComponentType) -> Result<Self, Self::Error> {
        match value {
            ChatComponentType::Primitive(text) => {
                Ok(Chat::text(text))
            }
            ChatComponentType::Array(array) => {
                let mut iterator = array.into_iter();
                let mut first = match iterator.next() {
                    Some(value) => value,
                    None => return Err(ChatComponentDeserializeErr::EmptyArray),
                };
                if iterator.len() != 0 {
                    first.children = iterator.as_slice().to_vec();
                }
                Ok(first)
            }
            ChatComponentType::Object(fake) => Ok(Chat::from(fake)),
        }
    }
}

impl Chat {
    /// Serialize this chat component to a JSON string.
    ///
    /// Serialization happens using [`serde_json`]. Newer style elements
    /// are automatically excluded if the provided version doesn't support
    /// these. When using a version that's 1.16 or above, [`HoverEvent`](crate::HoverEvent)
    /// uses a different data structure.
    ///
    /// # Example
    /// ```
    /// use mc_chat::{Chat, VERSION_1_8, VERSION_1_16};
    ///
    /// let chat = Chat::text("Sample text").font(Some("example_font"));
    /// let serialized_old = chat.serialize_str(VERSION_1_8).unwrap();
    /// assert_eq!(r#"{"text":"Sample text"}"#, serialized_old);
    ///
    /// let serialized_new = chat.serialize_str(VERSION_1_16).unwrap();
    /// assert_eq!(r#"{"text":"Sample text","font":"example_font"}"#, serialized_new);
    /// ```
    pub fn serialize_str(&self, version: i32) -> serde_json::Result<String> {
        serde_json::to_string(&SerializeChat {
            kind: (version, &self.kind).into(),
            style: (version, &self.style).into(),
            children: (version, &self.children),
        })
    }

    /// Serialize this chat component to JSON bytes.
    ///
    /// Serialization happens using [`serde_json`]. Newer style elements
    /// are automatically excluded if the provided version doesn't support
    /// these. When using a version that's 1.16 or above, [`HoverEvent`](crate::HoverEvent)
    /// uses a different data structure.
    ///
    /// # Example
    /// ```
    /// use mc_chat::{Chat, VERSION_1_8, VERSION_1_16};
    ///
    /// let chat = Chat::text("Sample text").font(Some("example_font"));
    /// let serialized_old = chat.serialize_vec(VERSION_1_8).unwrap();
    /// assert_eq!(&[123, 34, 116, 101, 120, 116, 34, 58, 34, 83, 97, 109, 112, 108, 101, 32, 116, 101, 120, 116, 34, 125], &serialized_old[..]);
    ///
    /// let serialized_new = chat.serialize_vec(VERSION_1_16).unwrap();
    /// assert_eq!(&[123, 34, 116, 101, 120, 116, 34, 58, 34, 83, 97, 109, 112, 108, 101, 32, 116,
    /// 101, 120, 116, 34, 44, 34, 102, 111, 110, 116, 34, 58, 34, 101, 120, 97, 109, 112, 108,
    /// 101, 95, 102, 111, 110, 116, 34, 125], &serialized_new[..]);
    /// ```
    pub fn serialize_vec(&self, version: i32) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(&SerializeChat {
            kind: (version, &self.kind).into(),
            style: (version, &self.style).into(),
            children: (version, &self.children),
        })
    }
}

#[derive(Serialize)]
pub(crate) struct SerializeTranslation<'a> {
    #[serde(rename = "translate")]
    key: &'a FrozenStr,
    #[serde(skip_serializing_if = "children_is_empty", default)]
    #[serde(serialize_with = "serialize_children")]
    with: (i32, &'a Vec<Chat>),
}

#[derive(Serialize)]
pub(crate) struct SerializeSelector<'a> {
    selector: &'a FrozenStr,
    #[serde(rename = "separator")]
    #[serde(skip_serializing_if = "version_option_none")]
    #[serde(serialize_with = "serialize_chat_option")]
    sep: (i32, &'a Option<Box<Chat>>),
}

pub(crate) fn version_option_none((_, value): &(i32, &Option<Box<Chat>>)) -> bool {
    value.is_none()
}

pub(crate) fn serialize_chat_option<S: Serializer>((version, chat): &(i32, &Option<Box<Chat>>), serializer: S) -> Result<S::Ok, S::Error> {
    match chat {
        Some(c) => SerializeChat {
            kind: (*version, &c.kind).into(),
            style: (*version, &c.style).into(),
            children: (*version, &c.children),
        }.serialize(serializer),
        None => serializer.serialize_none(),
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum SerializeComponent<'a> {
    Text(&'a TextComponent),
    Translation(SerializeTranslation<'a>),
    Score(&'a ScoreComponent),
    Selector(SerializeSelector<'a>),
    Keybind(&'a KeybindComponent),
}

impl<'a> From<(i32, &'a ComponentKind)> for SerializeComponent<'a> {
    fn from((version, component): (i32, &'a ComponentKind)) -> Self {
        match component {
            ComponentKind::Text(v) => Self::Text(v),
            ComponentKind::Translation(v) => Self::Translation(SerializeTranslation {
                key: &v.key,
                with: (version, &v.with),
            }),
            ComponentKind::Score(v) => Self::Score(v),
            ComponentKind::Selector(v) => Self::Selector(SerializeSelector {
                selector: &v.selector,
                sep: (version, &v.sep),
            }),
            ComponentKind::Keybind(v) => Self::Keybind(v),
        }
    }
}

#[derive(Serialize)]
pub(crate) struct SerializeChat<'a> {
    #[serde(flatten)]
    pub kind: SerializeComponent<'a>,
    #[serde(flatten)]
    pub style: StyleVersioned<'a>,
    #[serde(rename = "extra", skip_serializing_if = "children_is_empty", default)]
    #[serde(serialize_with = "serialize_children")]
    pub children: (i32, &'a Vec<Chat>),
}

fn serialize_children<S: Serializer>((version, children): &(i32, &Vec<Chat>), serializer: S) -> Result<S::Ok, S::Error> {
    let mut serializer = serializer.serialize_seq(Some(children.len()))?;
    for child in *children {
        serializer.serialize_element(&SerializeChat {
            kind: (*version, &child.kind).into(),
            style: (*version, &child.style).into(),
            children: (*version, &child.children),
        })?;
    }
    serializer.end()
}

fn children_is_empty((_, children): &(i32, &Vec<Chat>)) -> bool {
    children.is_empty()
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::VERSION_1_8;

    use super::*;

    #[test]
    pub fn serialize_chat_text() {
        let chat = Chat::text("Sample text");
        let serialized = chat.serialize_str(VERSION_1_8).unwrap();
        assert_eq!(r#"{"text":"Sample text"}"#, serialized);
    }

    #[test]
    pub fn deserialize_primitive() {
        let chat_orig = Chat::text("Sample text");

        let primitive = r#""Sample text""#;
        let chat: Chat = serde_json::from_str(primitive).unwrap();
        assert_eq!(chat_orig, chat);

        let value: Value = serde_json::from_str(primitive).unwrap();
        let chat: Chat = serde_json::from_value(value).unwrap();
        assert_eq!(chat_orig, chat);
    }

    #[test]
    pub fn deserialize_object() {
        let chat_orig = Chat::text("Sample text");

        let object = r#"{"text":"Sample text"}"#;
        let chat: Chat = serde_json::from_str(object).unwrap();
        assert_eq!(chat_orig, chat);

        let value: Value = serde_json::from_str(object).unwrap();
        let chat: Chat = serde_json::from_value(value).unwrap();
        assert_eq!(chat_orig, chat);
    }
}
