use crate::{ChatColor, ClickEvent, HoverEvent};
use serde::Serialize;

use erased_serde::serialize_trait_object;

pub trait Component: erased_serde::Serialize {
    fn get_siblings<'a>(&'a self) -> &'a Vec<Box<dyn Component>>;

    fn get_style(&self) -> &ComponentStyle;

    fn get_style_mut(&mut self) -> &mut ComponentStyle;

    fn append<'a>(&'a mut self, sibling: Box<dyn Component>);
}

serialize_trait_object!(Component);

#[derive(Serialize)]
pub struct ComponentStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlined: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<ChatColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertion: Option<String>,
    #[serde(rename = "clickEvent", skip_serializing_if = "Option::is_none")]
    pub click_event: Option<ClickEvent>,
    #[serde(rename = "hoverEvent", skip_serializing_if = "Option::is_none")]
    pub hover_event: Option<HoverEvent>
}

impl ComponentStyle {
    pub fn new() -> ComponentStyle {
        ComponentStyle {
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
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
        if self.bold.is_none() { self.bold = style.bold; }
        if self.italic.is_none() { self.italic = style.italic; }
        if self.underlined.is_none() { self.underlined = style.underlined; }
        if self.strikethrough.is_none() { self.strikethrough = style.strikethrough; }
        if self.obfuscated.is_none() { self.obfuscated = style.obfuscated; }
        if self.color.is_none() { self.color = style.color.clone(); }
        if self.insertion.is_none() { self.insertion = style.insertion.clone() }
    }

    pub fn reset(&mut self) {
        self.bold = None;
        self.italic = None;
        self.underlined = None;
        self.strikethrough = None;
        self.obfuscated = None;
        self.color = None;
        self.insertion = None;
        self.hover_event = None;
        self.click_event = None;
    }
}