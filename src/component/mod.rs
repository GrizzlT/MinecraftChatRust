use crate::style::ComponentStyle;
use std::{ops::{Deref, DerefMut}, borrow::Cow};

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

    pub fn text<T: Into<Cow<'static, str>>>(text: T, style: ComponentStyle) -> Self {
        ChatComponent {
            kind: ComponentType::Text(TextComponent::new(text)),
            style,
            siblings: vec![],
        }
    }

    pub fn key<T: Into<Cow<'static, str>>>(key: T, style: ComponentStyle) -> Self {
        ChatComponent {
            kind: ComponentType::Translation(TranslationComponent::new(key)),
            style,
            siblings: vec![],
        }
    }

    pub fn score<T: Into<Cow<'static, str>>, U: Into<Cow<'static, str>>>(
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

    pub fn selector<T: Into<Cow<'static, str>>>(selector: T, style: ComponentStyle) -> Self {
        ChatComponent {
            kind: ComponentType::Selector(SelectorComponent::new(selector)),
            style,
            siblings: vec![],
        }
    }

    pub fn keybind<T: Into<Cow<'static, str>>>(keybind: T, style: ComponentStyle) -> Self {
        ChatComponent {
            kind: ComponentType::Keybind(KeybindComponent::new(keybind)),
            style,
            siblings: vec![],
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextComponent {
    pub text: Cow<'static, str>,
}

impl TextComponent {
    pub fn new<T: Into<Cow<'static, str>>>(text: T) -> Self {
        TextComponent { text: text.into() }
    }

    pub fn text<T: Into<Cow<'static, str>>>(mut self, text: T) -> Self {
        self.text = text.into();
        self
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TranslationComponent {
    #[cfg_attr(feature = "serde", serde(rename = "translate"))]
    pub key: Cow<'static, str>,
    pub with: Vec<ChatComponent>,
}

impl TranslationComponent {
    pub fn new<T: Into<Cow<'static, str>>>(key: T) -> Self {
        TranslationComponent {
            key: key.into(),
            with: vec![],
        }
    }

    pub fn key<T: Into<Cow<'static, str>>>(mut self, key: T) -> Self {
        self.key = key.into();
        self
    }

    pub fn argument(mut self, component: ChatComponent) -> Self {
        self.with.push(component);
        self
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScoreComponent {
    pub name: Cow<'static, str>,
    pub objective: Cow<'static, str>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub value: Option<Cow<'static, str>>,
}

impl ScoreComponent {
    pub fn new<T: Into<Cow<'static, str>>, U: Into<Cow<'static, str>>>(name: T, objective: U) -> Self {
        ScoreComponent {
            name: name.into(),
            objective: objective.into(),
            value: None,
        }
    }

    pub fn name<T: Into<Cow<'static, str>>>(mut self, name: T) -> Self {
        self.name = name.into();
        self
    }

    pub fn objective<T: Into<Cow<'static, str>>>(mut self, objective: T) -> Self {
        self.objective = objective.into();
        self
    }

    pub fn value<T: Into<Cow<'static, str>>>(mut self, value: Option<T>) -> Self {
        self.value = value.map(|value| value.into());
        self
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SelectorComponent {
    pub selector: Cow<'static, str>,
}

impl SelectorComponent {
    pub fn new<T: Into<Cow<'static, str>>>(selector: T) -> Self {
        SelectorComponent {
            selector: selector.into(),
        }
    }

    pub fn selector<T: Into<Cow<'static, str>>>(mut self, selector: T) -> Self {
        self.selector = selector.into();
        self
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeybindComponent {
    pub keybind: Cow<'static, str>,
}

impl KeybindComponent {
    pub fn new<T: Into<Cow<'static, str>>>(keybind: T) -> Self {
        KeybindComponent {
            keybind: keybind.into(),
        }
    }

    pub fn keybind<T: Into<Cow<'static, str>>>(mut self, keybind: T) -> Self {
        self.keybind = keybind.into();
        self
    }
}
