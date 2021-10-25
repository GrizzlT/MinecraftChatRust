use crate::{Component, ComponentStyle};

#[cfg(feature = "serde-support")]
use serde::Serialize;

/// A Literal Text Component.
#[cfg_attr(feature = "serde-support", derive(Serialize))]
pub struct TextComponent {
    text: String,
    #[cfg_attr(feature = "serde-support", serde(flatten))]
    style: ComponentStyle,
    #[cfg_attr(feature = "serde-support", serde(rename = "extra", skip_serializing_if = "Vec::is_empty"))]
    siblings: Vec<Box<dyn Component>>
}

impl TextComponent {
    pub fn from_text<T: Into<String>>(text: T, style: ComponentStyle) -> TextComponent {
        TextComponent {
            text: text.into(),
            style,
            siblings: vec![]
        }
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }
}

impl Component for TextComponent {
    fn get_children<'a>(&'a self) -> &'a Vec<Box<dyn Component>> {
        &self.siblings
    }

    fn get_style(&self) -> &ComponentStyle {
        &self.style
    }

    fn get_style_mut(&mut self) -> &mut ComponentStyle {
        &mut self.style
    }

    fn append(&mut self, child: Box<dyn Component>) {
        self.siblings.push(child)
    }
}

/// A Text Component that uses a translation key and arguments.
#[cfg_attr(feature = "serde-support", derive(Serialize))]
pub struct TranslatableComponent {
    key: String,
    #[cfg_attr(feature = "serde-support", serde(skip_serializing_if = "Vec::is_empty"))]
    with: Vec<Box<dyn Component>>,
    #[cfg_attr(feature = "serde-support", serde(flatten))]
    style: ComponentStyle,
    #[cfg_attr(feature = "serde-support", serde(rename = "extra", skip_serializing_if = "Vec::is_empty"))]
    siblings: Vec<Box<dyn Component>>
}

impl TranslatableComponent {
    pub fn from_key<T: Into<String>>(key: T, style: ComponentStyle) -> TranslatableComponent {
        TranslatableComponent {
            key: key.into(),
            with: vec![],
            style,
            siblings: vec![]
        }
    }

    pub fn key(mut self, key: String) -> Self {
        self.key = key;
        self
    }

    pub fn add_arg(mut self, arg: Box<dyn Component>) -> Self {
        self.with.push(arg);
        self
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_args(&self) -> &Vec<Box<dyn Component>> {
        &self.with
    }
}

impl Component for TranslatableComponent {
    fn get_children<'a>(&'a self) -> &'a Vec<Box<dyn Component>> {
        &self.siblings
    }

    fn get_style(&self) -> &ComponentStyle {
        &self.style
    }

    fn get_style_mut(&mut self) -> &mut ComponentStyle {
        &mut self.style
    }

    fn append(&mut self, child: Box<dyn Component>) {
        self.siblings.push(child)
    }
}