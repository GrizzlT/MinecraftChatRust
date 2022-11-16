use std::ops::{Deref, DerefMut};

use crate::{style::ComponentStyle, freeze::FreezeStr};

#[cfg(feature = "serde")]
mod serde_support;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The central building block of Minecraft's JSON message format.
///
/// In Rust, this consists of a type ([`ComponentType`])
/// , a style ([`ComponentStyle`]) and a list of other `ChatComponent`s
/// that inherit the style of their parent.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "serde_support::ChatComponentType")
)]
pub struct ChatComponent {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub kind: ComponentType,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub style: ComponentStyle,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "extra", skip_serializing_if = "Vec::is_empty", default)
    )]
    pub siblings: Vec<ChatComponent>,
}

impl ChatComponent {
    pub fn component(kind: ComponentType, style: ComponentStyle) -> Self {
        ChatComponent {
            kind,
            style,
            siblings: vec![],
        }
    }

    pub fn text<T: Into<FreezeStr>>(text: T, style: ComponentStyle) -> Self {
        ChatComponent {
            kind: ComponentType::Text(TextComponent::new(text)),
            style,
            siblings: vec![],
        }
    }

    pub fn key<T: Into<FreezeStr>>(key: T, style: ComponentStyle) -> Self {
        ChatComponent {
            kind: ComponentType::Translation(TranslationComponent::new(key)),
            style,
            siblings: vec![],
        }
    }

    pub fn score<T: Into<FreezeStr>, U: Into<FreezeStr>>(
        name: T,
        objective: U,
        style: ComponentStyle,
    ) -> Self {
        ChatComponent {
            kind: ComponentType::Score(ScoreComponent::new(name, objective)),
            style,
            siblings: vec![],
        }
    }

    pub fn selector<T: Into<FreezeStr>>(selector: T, style: ComponentStyle) -> Self {
        ChatComponent {
            kind: ComponentType::Selector(SelectorComponent::new(selector)),
            style,
            siblings: vec![],
        }
    }

    pub fn keybind<T: Into<FreezeStr>>(keybind: T, style: ComponentStyle) -> Self {
        ChatComponent {
            kind: ComponentType::Keybind(KeybindComponent::new(keybind)),
            style,
            siblings: vec![],
        }
    }
    
    pub fn freeze(&mut self) {
        self.kind.freeze();
        self.style.freeze();
        for sibling in &mut self.siblings {
            sibling.freeze();
        }
    }
}

impl Deref for ChatComponent {
    type Target = ComponentStyle;

    fn deref(&self) -> &Self::Target {
        &self.style
    }
}

impl DerefMut for ChatComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.style
    }
}

/// The different kinds of components Minecraft chat messages
/// can be made up of. One component (`storage`-component, since 1.15) is missing,
/// further research and contributions on this would be appreciated!
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ComponentType {
    Text(TextComponent),
    Translation(TranslationComponent),
    /// # Warning
    /// Since **1.8**!
    ///
    /// This crate does not check any version,
    /// it is up to the user to deal with this safely!
    Score(ScoreComponent),
    /// # Warning
    /// Since **1.8** and **Client-To-Server** only!
    ///
    /// This crate does not check these constraints,
    /// it is up to the user to deal with this safely!
    Selector(SelectorComponent),
    /// # Warning
    /// Since **1.12**!
    ///
    /// This crate does not check any version,
    /// it is up to the user to deal with this safely!
    Keybind(KeybindComponent),
    // TODO: research the `storage` component (since 1.15)
}

impl ComponentType {
    pub fn freeze(&mut self) {
        match self {
            Self::Text(v) => v.freeze(),
            Self::Translation(v) => v.freeze(),
            Self::Score(v) => v.freeze(),
            Self::Selector(v) => v.freeze(),
            Self::Keybind(v) => v.freeze(),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextComponent {
    pub text: FreezeStr,
}

impl TextComponent {
    pub fn new<T: Into<FreezeStr>>(text: T) -> Self {
        TextComponent { text: text.into() }
    }

    pub fn text<T: Into<FreezeStr>>(mut self, text: T) -> Self {
        self.text = text.into();
        self
    }

    pub fn freeze(&mut self) {
        self.text.freeze();
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TranslationComponent {
    #[cfg_attr(feature = "serde", serde(rename = "translate"))]
    pub key: FreezeStr,
    pub with: Vec<ChatComponent>,
}

impl TranslationComponent {
    pub fn new<T: Into<FreezeStr>>(key: T) -> Self {
        TranslationComponent {
            key: key.into(),
            with: vec![],
        }
    }

    pub fn key<T: Into<FreezeStr>>(mut self, key: T) -> Self {
        self.key = key.into();
        self
    }

    pub fn argument(mut self, component: ChatComponent) -> Self {
        self.with.push(component);
        self
    }

    pub fn freeze(&mut self) {
        self.key.freeze();
        for arg in &mut self.with {
            arg.freeze();
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScoreComponent {
    pub name: FreezeStr,
    pub objective: FreezeStr,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub value: Option<FreezeStr>,
}

impl ScoreComponent {
    pub fn new<T: Into<FreezeStr>, U: Into<FreezeStr>>(name: T, objective: U) -> Self {
        ScoreComponent {
            name: name.into(),
            objective: objective.into(),
            value: None,
        }
    }

    pub fn name<T: Into<FreezeStr>>(mut self, name: T) -> Self {
        self.name = name.into();
        self
    }

    pub fn objective<T: Into<FreezeStr>>(mut self, objective: T) -> Self {
        self.objective = objective.into();
        self
    }

    pub fn value<T: Into<FreezeStr>>(mut self, value: Option<T>) -> Self {
        self.value = value.map(|value| value.into());
        self
    }

    pub fn freeze(&mut self) {
        self.name.freeze();
        self.objective.freeze();
        if let Some(value) = &mut self.value {
            value.freeze();
        }

    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SelectorComponent {
    pub selector: FreezeStr,
}

impl SelectorComponent {
    pub fn new<T: Into<FreezeStr>>(selector: T) -> Self {
        SelectorComponent {
            selector: selector.into(),
        }
    }

    pub fn selector<T: Into<FreezeStr>>(mut self, selector: T) -> Self {
        self.selector = selector.into();
        self
    }

    pub fn freeze(&mut self) {
        self.selector.freeze();
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeybindComponent {
    pub keybind: FreezeStr,
}

impl KeybindComponent {
    pub fn new<T: Into<FreezeStr>>(keybind: T) -> Self {
        KeybindComponent {
            keybind: keybind.into(),
        }
    }

    pub fn keybind<T: Into<FreezeStr>>(mut self, keybind: T) -> Self {
        self.keybind = keybind.into();
        self
    }

    pub fn freeze(&mut self) {
        self.keybind.freeze();
    }
}
