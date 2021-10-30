use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use crate::ComponentType;
use serde::Deserialize;

use crate::style::ComponentStyle;

use super::ChatComponent;

#[derive(Deserialize)]
pub(crate) struct FakeChatComponent {
    #[serde(flatten)]
    kind: ComponentType,
    #[serde(flatten)]
    style: ComponentStyle,
    #[serde(rename = "extra", default)]
    siblings: Vec<ChatComponent>,
}

impl From<FakeChatComponent> for ChatComponent {
    fn from(component: FakeChatComponent) -> Self {
        ChatComponent {
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
    Array(Vec<ChatComponent>),
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

impl TryFrom<ChatComponentType> for ChatComponent {
    type Error = ChatComponentDeserializeErr;

    fn try_from(value: ChatComponentType) -> Result<Self, Self::Error> {
        match value {
            ChatComponentType::Primitive(text) => {
                Ok(ChatComponent::from_text(text, ComponentStyle::v1_16()))
            }
            ChatComponentType::Array(mut array) => {
                let mut iterator = array.drain(..);
                let mut first = match iterator.next() {
                    Some(value) => value,
                    None => return Err(ChatComponentDeserializeErr::EmptyArray),
                };
                if iterator.len() != 0 {
                    first.siblings = iterator.as_slice().to_vec();
                }
                Ok(first)
            }
            ChatComponentType::Object(fake) => Ok(ChatComponent::from(fake)),
        }
    }
}
