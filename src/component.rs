use crate::{style::Style, freeze::FrozenStr, TextColor, HoverEvent, ClickEvent};

#[cfg(feature = "serde")]
pub(crate) mod serde_support;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A Minecraft chat/text component.
///
/// There are different [`ComponentKind`] kinds.
/// Each text component has a [`Style`]
/// and optional child text components. The children
/// of a text component inherit the component's style.
/// This inherited style can be overwritten.
///
/// # Example
/// ```
/// use mc_chat::{Chat, TextColor};
///
/// let chat = Chat::text("This is a bold and italic ")
///     .bold(true)
///     .italic(true)
///     .child(Chat::text("text").color(TextColor::Green));
///
/// assert_eq!("{\"text\":\"This is a bold and italic \",\"bold\":true,\"italic\":true,\"extra\":[{\"text\":\"text\",\"color\":\"green\"}]}", chat.serialize_str(47).unwrap());
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "serde_support::ChatComponentType")
)]
pub struct Chat {
    /// The type of this component
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub kind: ComponentKind,
    /// The style of this component.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub style: Style,
    /// The children of this component.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "extra", skip_serializing_if = "Vec::is_empty", default)
    )]
    pub children: Vec<Chat>,
}

impl Chat {
    /// Creates a new chat component based on a give [`ComponentKind`].
    ///
    /// # Example
    /// ```
    /// use mc_chat::{Chat, ComponentKind, TextComponent};
    ///
    /// let chat = Chat::component(TextComponent::new("Chat component"));
    ///
    /// assert_eq!("{\"text\":\"Chat component\"}", chat.serialize_str(47).unwrap());
    /// ```
    pub fn component<C>(kind: C) -> Self
    where
        C: Into<ComponentKind>,
    {
        Chat {
            kind: kind.into(),
            style: Default::default(),
            children: vec![],
        }
    }

    /// Creates a new [`TextComponent`].
    ///
    /// # Example
    /// ```
    /// use mc_chat::Chat;
    ///
    /// let chat = Chat::text("Literal text.");
    ///
    /// assert_eq!("{\"text\":\"Literal text.\"}", chat.serialize_str(47).unwrap());
    /// ```
    pub fn text<T: Into<FrozenStr>>(text: T) -> Self {
        Chat::component(TextComponent::new(text))
    }

    /// Creates a new [`TranslationComponent`].
    ///
    /// # Example
    /// ```
    /// use mc_chat::Chat;
    ///
    /// // display name of a bow
    /// let chat = Chat::key("item.bow.name");
    ///
    /// assert_eq!("{\"translate\":\"item.bow.name\"}", chat.serialize_str(47).unwrap());
    /// ```
    pub fn key<T: Into<FrozenStr>>(key: T) -> Self {
        Chat::component(TranslationComponent::new(key))
    }

    /// Creates a new [`ScoreComponent`].
    ///
    /// # Example
    /// ```
    /// use mc_chat::Chat;
    ///
    /// // show the amount of stars the reader has gained
    /// let chat = Chat::score("*", "stars_gained");
    ///
    /// assert_eq!("{\"score\":{\"name\":\"*\",\"objective\":\"stars_gained\"}}", chat.serialize_str(47).unwrap());
    /// ```
    pub fn score<T, U>(name: T, objective: U) -> Self
    where
        T: Into<FrozenStr>,
        U: Into<FrozenStr>,
    {
        Chat::component(ScoreComponent::new(name, objective))
    }

    /// Creates a new [`SelectorComponent`].
    ///
    /// # Example
    /// ```
    /// use mc_chat::Chat;
    ///
    /// let chat = Chat::selector("@e[type=Zombie,limit=1]", None);
    ///
    /// assert_eq!("{\"selector\":\"@e[type=Zombie,limit=1]\"}", chat.serialize_str(47).unwrap());
    /// ```
    pub fn selector<T: Into<FrozenStr>>(selector: T, sep: Option<Chat>) -> Self {
        Chat::component(SelectorComponent::new(selector, sep))
    }

    /// Creates a new [`KeybindComponent`].
    ///
    /// # Example
    /// ```
    /// use mc_chat::Chat;
    ///
    /// let chat = Chat::keybind("key.inventory");
    ///
    /// assert_eq!("{\"keybind\":\"key.inventory\"}", chat.serialize_str(47).unwrap());
    /// ```
    pub fn keybind<T: Into<FrozenStr>>(keybind: T) -> Self {
        Chat::component(KeybindComponent::new(keybind))
    }

    /// Adds a child component to this chat component.
    ///
    /// # Example
    /// ```
    /// use mc_chat::{Chat, TextColor};
    ///
    /// let chat = Chat::text("The color of the child's ")
    ///     .color(TextColor::Green)
    ///     .child(Chat::text(" text will also be green."));
    /// ```
    pub fn child(mut self, child: Chat) -> Self {
        self.children.push(child);
        self
    }

    pub fn color(mut self, color: TextColor) -> Self {
        self.style.color(color);
        self
    }

    pub fn bold(mut self, bold: bool) -> Self {
        self.style.bold(bold);
        self
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.style.italic(italic);
        self
    }

    pub fn underlined(mut self, underlined: bool) -> Self {
        self.style.underlined(underlined);
        self
    }

    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.style.strikethrough(strikethrough);
        self
    }

    pub fn obfuscated(mut self, obfuscated: bool) -> Self {
        self.style.obfuscated(obfuscated);
        self
    }

    pub fn font<T: Into<FrozenStr>>(mut self, font: Option<T>) -> Self {
        self.style.font(font);
        self
    }

    pub fn insertion<T: Into<FrozenStr>>(mut self, insertion: Option<T>) -> Self {
        self.style.insertion(insertion);
        self
    }

    pub fn click(mut self, click_event: Option<ClickEvent>) -> Self {
        self.style.click(click_event);
        self
    }

    pub fn hover(mut self, hover_event: Option<HoverEvent>) -> Self {
        self.style.hover(hover_event);
        self
    }
}

/// The different kinds of components Minecraft chat messages
/// can be made up of. One component (`storage`-component, since 1.15) is missing,
/// further research and contributions on this would be appreciated!
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ComponentKind {
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

impl From<TextComponent> for ComponentKind {
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

impl From<TranslationComponent> for ComponentKind {
    fn from(value: TranslationComponent) -> Self {
        Self::Translation(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "serde_support::SerializeScore"))]
#[cfg_attr(feature = "serde", serde(into = "serde_support::SerializeScore"))]
pub struct ScoreComponent {
    pub name: FrozenStr,
    pub objective: FrozenStr,
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

impl From<ScoreComponent> for ComponentKind {
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

impl From<SelectorComponent> for ComponentKind {
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

impl From<KeybindComponent> for ComponentKind {
    fn from(value: KeybindComponent) -> Self {
        Self::Keybind(value)
    }
}
