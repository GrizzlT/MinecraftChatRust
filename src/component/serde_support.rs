use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use crate::{Component, TextComponent, ScoreComponent, KeybindComponent};
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

#[derive(Deserialize)]
pub(crate) struct FakeChatComponent {
    #[serde(flatten)]
    kind: Component,
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

#[derive(Deserialize)]
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
    pub fn serialize_str(&self, version: i32) -> serde_json::Result<String> {
        serde_json::to_string(&SerializeChat {
            kind: (version, &self.kind).into(),
            style: (version, &self.style).into(),
            children: (version, &self.children),
        })
    }

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

impl<'a> From<(i32, &'a Component)> for SerializeComponent<'a> {
    fn from((version, component): (i32, &'a Component)) -> Self {
        match component {
            Component::Text(v) => Self::Text(v),
            Component::Translation(v) => Self::Translation(SerializeTranslation {
                key: &v.key,
                with: (version, &v.with),
            }),
            Component::Score(v) => Self::Score(v),
            Component::Selector(v) => Self::Selector(SerializeSelector {
                selector: &v.selector,
                sep: (version, &v.sep),
            }),
            Component::Keybind(v) => Self::Keybind(v),
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
