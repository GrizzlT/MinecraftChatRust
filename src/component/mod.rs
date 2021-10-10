use crate::{ChatColor, ClickEvent, HoverEvent};
use serde::Serialize;

pub trait Component {
    fn get_siblings<'a>(&'a self) -> &'a Vec<Box<dyn Component>>;

    fn get_style(&self) -> &ComponentStyle;

    fn get_style_mut(&mut self) -> &mut ComponentStyle;

    fn append<'a>(&'a mut self, sibling: Box<dyn Component>);
}

#[derive(Serialize)]
pub struct ComponentStyle {
    pub bold: bool,
    pub italic: bool,
    pub underlined: bool,
    pub strikethrough: bool,
    pub obfuscated: bool,
    pub color: Option<ChatColor>,
    pub insertion: Option<String>,
    pub click_event: Option<ClickEvent>,
    pub hover_event: Option<HoverEvent>
}

impl ComponentStyle {
    pub fn new() -> ComponentStyle {
        ComponentStyle {
            bold: false,
            italic: false,
            underlined: false,
            strikethrough: false,
            obfuscated: false,
            color: None,
            insertion: None,
            click_event: None,
            hover_event: None
        }
    }

    ///
    /// This clones the given style except for HoverEvent and ClickEvent.
    pub fn apply_style(&mut self, style: &ComponentStyle) {
        self.bold = style.bold;
        self.italic = style.italic;
        self.underlined = style.underlined;
        self.strikethrough = style.strikethrough;
        self.obfuscated = style.obfuscated;
        self.color = style.color.clone();
        self.insertion = style.insertion.clone();
    }

    pub fn merge_style(&mut self, style: &ComponentStyle) {
        if !self.bold { self.bold = style.bold; }
        if !self.italic { self.italic = style.italic; }
        if !self.underlined { self.underlined = style.underlined; }
        if !self.strikethrough { self.strikethrough = style.strikethrough; }
        if !self.obfuscated { self.obfuscated = style.obfuscated; }
        if self.color.is_none() { self.color = style.color.clone(); }
        if self.insertion.is_none() { self.insertion = style.insertion.clone() }
    }

    pub fn reset(&mut self) {
        self.bold = false;
        self.italic = false;
        self.underlined = false;
        self.strikethrough = false;
        self.obfuscated = false;
        self.color = None;
        self.insertion = None;
        self.hover_event = None;
        self.click_event = None;
    }
}