use std::ops::{Deref, DerefMut};

use crate::{style::Style, freeze::FrozenStr};

#[cfg(feature = "serde")]
pub(crate) mod serde_support;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The central building block of Minecraft's JSON message format.
///
/// In Rust, this consists of a ([`Component`])
/// , a ([`Style`]) and a list of other `Chat` objects
/// that inherit the style from their parent.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "serde_support::ChatComponentType")
)]
pub struct Chat {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub kind: Component,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub style: Style,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "extra", skip_serializing_if = "Vec::is_empty", default)
    )]
    pub siblings: Vec<Chat>,
}

impl Chat {
    pub fn component<C>(kind: C) -> Self
    where
        C: Into<Component>,
    {
        Chat {
            kind: kind.into(),
            style: Default::default(),
            siblings: vec![],
        }
    }

    pub fn text<T: Into<FrozenStr>>(text: T) -> Self {
        Chat::component(TextComponent::new(text))
    }

    pub fn key<T: Into<FrozenStr>>(key: T) -> Self {
        Chat::component(TranslationComponent::new(key))
    }

    pub fn score<T, U>(name: T, objective: U) -> Self
    where
        T: Into<FrozenStr>,
        U: Into<FrozenStr>,
    {
        Chat::component(ScoreComponent::new(name, objective))
    }

    pub fn selector<T: Into<FrozenStr>>(selector: T, sep: Option<Chat>) -> Self {
        Chat::component(SelectorComponent::new(selector, sep))
    }

    pub fn keybind<T: Into<FrozenStr>>(keybind: T) -> Self {
        Chat::component(KeybindComponent::new(keybind))
    }

    pub fn sibling(&mut self, sibling: Chat) -> &mut Self {
        self.siblings.push(sibling);
        self
    }
}

impl Deref for Chat {
    type Target = Style;

    fn deref(&self) -> &Self::Target {
        &self.style
    }
}

impl DerefMut for Chat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.style
    }
}

/// The different kinds of components Minecraft chat messages
/// can be made up of. One component (`storage`-component, since 1.15) is missing,
/// further research and contributions on this would be appreciated!
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Component {
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
    // TODO: research the `nbt` values
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextComponent {
    pub text: FrozenStr,
}

impl TextComponent {
    pub fn new<T: Into<FrozenStr>>(text: T) -> Self {
        TextComponent { text: text.into() }
    }

    pub fn text<T: Into<FrozenStr>>(mut self, text: T) -> Self {
        self.text = text.into();
        self
    }
}

impl From<TextComponent> for Component {
    fn from(value: TextComponent) -> Self {
        Self::Text(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct TranslationComponent {
    #[cfg_attr(feature = "serde", serde(rename = "translate"))]
    pub key: FrozenStr,
    pub with: Vec<Chat>,
}

impl TranslationComponent {
    pub fn new<T: Into<FrozenStr>>(key: T) -> Self {
        TranslationComponent {
            key: key.into(),
            with: vec![],
        }
    }

    pub fn key<T: Into<FrozenStr>>(mut self, key: T) -> Self {
        self.key = key.into();
        self
    }

    pub fn argument(mut self, component: Chat) -> Self {
        self.with.push(component);
        self
    }
}

impl From<TranslationComponent> for Component {
    fn from(value: TranslationComponent) -> Self {
        Self::Translation(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScoreComponent {
    pub name: FrozenStr,
    pub objective: FrozenStr,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub value: Option<FrozenStr>,
}

impl ScoreComponent {
    pub fn new<T: Into<FrozenStr>, U: Into<FrozenStr>>(name: T, objective: U) -> Self {
        ScoreComponent {
            name: name.into(),
            objective: objective.into(),
            value: None,
        }
    }

    pub fn name<T: Into<FrozenStr>>(mut self, name: T) -> Self {
        self.name = name.into();
        self
    }

    pub fn objective<T: Into<FrozenStr>>(mut self, objective: T) -> Self {
        self.objective = objective.into();
        self
    }

    pub fn value<T: Into<FrozenStr>>(mut self, value: Option<T>) -> Self {
        self.value = value.map(|value| value.into());
        self
    }
}

impl From<ScoreComponent> for Component {
    fn from(value: ScoreComponent) -> Self {
        Self::Score(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct SelectorComponent {
    pub selector: FrozenStr,
    pub sep: Option<Box<Chat>>,
}

impl SelectorComponent {
    pub fn new<T: Into<FrozenStr>>(selector: T, sep: Option<Chat>) -> Self {
        SelectorComponent {
            selector: selector.into(),
            sep: sep.map(Box::new),
        }
    }

    pub fn selector<T: Into<FrozenStr>>(mut self, selector: T) -> Self {
        self.selector = selector.into();
        self
    }

    pub fn sep(mut self, sep: Chat) -> Self {
        self.sep = Some(Box::new(sep));
        self
    }
}

impl From<SelectorComponent> for Component {
    fn from(value: SelectorComponent) -> Self {
        Self::Selector(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeybindComponent {
    pub keybind: FrozenStr,
}

impl KeybindComponent {
    pub fn new<T: Into<FrozenStr>>(keybind: T) -> Self {
        KeybindComponent {
            keybind: keybind.into(),
        }
    }

    pub fn keybind<T: Into<FrozenStr>>(mut self, keybind: T) -> Self {
        self.keybind = keybind.into();
        self
    }
}

impl From<KeybindComponent> for Component {
    fn from(value: KeybindComponent) -> Self {
        Self::Keybind(value)
    }
}
