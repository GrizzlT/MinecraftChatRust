use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use crate::Component;
use serde::Deserialize;

use crate::style::Style;

use super::Chat;

#[derive(Deserialize)]
pub(crate) struct FakeChatComponent {
    #[serde(flatten)]
    kind: Component,
    #[serde(flatten)]
    style: Style,
    #[serde(rename = "extra", default)]
    siblings: Vec<Chat>,
}

#[doc(hidden)]
impl From<FakeChatComponent> for Chat {
    fn from(component: FakeChatComponent) -> Self {
        Chat {
            kind: component.kind,
            style: component.style,
            siblings: component.siblings,
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
                Ok(Chat::text(text, Style::v1_16()))
            }
            ChatComponentType::Array(array) => {
                let mut iterator = array.into_iter();
                let mut first = match iterator.next() {
                    Some(value) => value,
                    None => return Err(ChatComponentDeserializeErr::EmptyArray),
                };
                if iterator.len() != 0 {
                    first.siblings = iterator.as_slice().to_vec();
                }
                Ok(first)
            }
            ChatComponentType::Object(fake) => Ok(Chat::from(fake)),
        }
    }
}
