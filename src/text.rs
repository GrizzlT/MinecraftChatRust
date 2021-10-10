use crate::{Component, ComponentStyle};

use serde::Serialize;

#[derive(Serialize)]
pub struct TextComponent {
    text: String,
    style: ComponentStyle,
    siblings: Vec<Box<dyn Component>>
}

impl TextComponent {
    pub fn from_text(text: String) -> TextComponent {
        TextComponent {
            text,
            style: ComponentStyle::new(),
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
    fn get_siblings<'a>(&'a self) -> &'a Vec<Box<dyn Component>> {
        &self.siblings
    }

    fn get_style(&self) -> &ComponentStyle {
        &self.style
    }

    fn get_style_mut(&mut self) -> &mut ComponentStyle {
        &mut self.style
    }

    fn append<'a>(&'a mut self, sibling: Box<dyn Component>) {
        self.siblings.push(sibling)
    }
}

pub struct TranslatableComponent {
    key: String,
    with: Vec<Box<dyn Component>>,
    style: ComponentStyle,
    siblings: Vec<Box<dyn Component>>
}

impl TranslatableComponent {
    pub fn from_key(key: String) -> TranslatableComponent {
        TranslatableComponent {
            key,
            with: vec![],
            style: ComponentStyle::new(),
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
    fn get_siblings<'a>(&'a self) -> &'a Vec<Box<dyn Component>> {
        &self.siblings
    }

    fn get_style(&self) -> &ComponentStyle {
        &self.style
    }

    fn get_style_mut(&mut self) -> &mut ComponentStyle {
        &mut self.style
    }

    fn append<'a>(&'a mut self, sibling: Box<dyn Component>) {
        self.siblings.push(sibling)
    }
}