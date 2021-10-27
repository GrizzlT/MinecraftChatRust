use crate::{ChatColor, ClickEvent, HoverEvent};

///
/// Represents a text object from minecraft that
/// can be serialized and deserialized into a JSON-message
/// suitable for sending across the minecraft protocol.
/// (see [Wiki.vg](https://wiki.vg/Chat))
#[cfg(feature = "serde-support")]
pub trait Component: erased_serde::Serialize {
    ///
    /// Fetches the children of this component.
    ///
    /// This is equal to the `"extra"` element in the JSON format.
    fn get_children<'a>(&'a self) -> &'a Vec<Box<dyn Component>>;

    /// Gets the style component associated with this component.
    fn get_style(&self) -> &ComponentStyle;

    /// Gets the style component associated with this component.
    fn get_style_mut(&mut self) -> &mut ComponentStyle;

    ///
    /// Adds a child to this component.
    /// This will result in an addition to the `"extra"` element in the JSON format.
    fn append(&mut self, child: Box<dyn Component>);
}

///
/// Represents a text object from minecraft that
/// can be serialized and deserialized into a JSON-message
/// suitable for sending across the minecraft protocol.
/// (see [Wiki.vg](https://wiki.vg/Chat))
#[cfg(not(feature = "serde-support"))]
pub trait Component {
    ///
    /// Fetches the children of this component.
    ///
    /// This is equal to the `"extra"` element in the JSON format.
    fn get_children<'a>(&'a self) -> &'a Vec<Box<dyn Component>>;

    /// Gets the style component associated with this component.
    fn get_style(&self) -> &ComponentStyle;

    /// Gets the style component associated with this component.
    fn get_style_mut(&mut self) -> &mut ComponentStyle;

    ///
    /// Adds a child to this component.
    /// This will result in an addition to the `"extra"` element in the JSON format.
    fn append(&mut self, child: Box<dyn Component>);
}

/// The central struct containing all style information about a [`Component`].
pub struct ComponentStyle {
    pub(crate) version: i32,
    pub(crate) bold: Option<bool>,
    pub(crate) italic: Option<bool>,
    pub(crate) underlined: Option<bool>,
    pub(crate) strikethrough: Option<bool>,
    pub(crate) obfuscated: Option<bool>,
    pub(crate) color: Option<crate::ChatColor>,
    /// This field is ignored for versions older than 1.8
    pub(crate) insertion: Option<String>,
    /// This field is ignored for versions older than 1.16
    pub(crate) font: Option<String>,
    pub(crate) click_event: Option<crate::ClickEvent>,
    pub(crate) hover_event: Option<crate::HoverEvent>,
}

pub trait ComponentStyleEditable {
    fn color(&mut self, color: Option<ChatColor>);

    fn color_if_absent(&mut self, color: ChatColor);

    fn bold(&mut self, bold: bool);

    fn italic(&mut self, italic: bool);

    fn underlined(&mut self, underlined: bool);

    fn strikethrough(&mut self, strikethrough: bool);

    fn obfuscated(&mut self, obfuscated: bool);

    fn font(&mut self, font: Option<String>);

    fn insertion(&mut self, insertion: Option<String>);

    fn click_event(&mut self, click_event: Option<ClickEvent>);

    fn hover_event(&mut self, hover_event: Option<HoverEvent>);

    fn get_color(&self) -> Option<&ChatColor>;

    fn get_bold(&self) -> Option<bool>;

    fn get_italic(&self) -> Option<bool>;

    fn get_underlined(&self) -> Option<bool>;

    fn get_strikethrough(&self) -> Option<bool>;

    fn get_obfuscated(&self) -> Option<bool>;

    fn get_font(&self) -> Option<&String>;

    fn get_insertion(&self) -> Option<&String>;

    fn get_click_event(&self) -> Option<&ClickEvent>;

    fn get_hover_event(&self) -> Option<&HoverEvent>;
}

impl ComponentStyle {
    pub fn v1_7() -> ComponentStyle {
        ComponentStyle {
            version: 4,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            color: None,
            insertion: None,
            font: None,
            click_event: None,
            hover_event: None,
        }
    }

    pub fn v1_8() -> ComponentStyle {
        ComponentStyle {
            version: 47,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            color: None,
            insertion: None,
            font: None,
            click_event: None,
            hover_event: None,
        }
    }

    pub fn v1_15() -> ComponentStyle {
        ComponentStyle {
            version: 573,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            color: None,
            insertion: None,
            font: None,
            click_event: None,
            hover_event: None,
        }
    }

    pub fn v1_16() -> ComponentStyle {
        ComponentStyle {
            version: 735,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            color: None,
            insertion: None,
            font: None,
            click_event: None,
            hover_event: None,
        }
    }

    /// Assigns all fields except [`ComponentStyle::hover_event`] and [`ComponentStyle::click_event`]
    /// to the values of the specified style.
    pub fn apply_style(&mut self, style: &ComponentStyle) {
        self.bold = style.bold;
        self.italic = style.italic;
        self.underlined = style.underlined;
        self.strikethrough = style.strikethrough;
        self.obfuscated = style.obfuscated;
        self.color = style.color.clone();
        self.insertion = style.insertion.clone();
        self.font = style.insertion.clone();
    }

    /// Resets all fields to default (being [`None`]).
    pub fn reset(&mut self) {
        self.bold = None;
        self.italic = None;
        self.underlined = None;
        self.strikethrough = None;
        self.obfuscated = None;
        self.color = None;
        self.insertion = None;
        self.font = None;
        self.click_event = None;
        self.hover_event = None;
    }
}

impl ComponentStyleEditable for ComponentStyle {
    fn color(&mut self, color: Option<ChatColor>) {
        self.color = color;
    }

    fn color_if_absent(&mut self, color: ChatColor) {
        if self.color.is_none() {
            self.color = Some(color);
        }
    }

    fn bold(&mut self, bold: bool) {
        self.bold = Some(bold);
    }

    fn italic(&mut self, italic: bool) {
        self.italic = Some(italic);
    }

    fn underlined(&mut self, underlined: bool) {
        self.underlined = Some(underlined);
    }

    fn strikethrough(&mut self, strikethrough: bool) {
        self.strikethrough = Some(strikethrough);
    }

    fn obfuscated(&mut self, obfuscated: bool) {
        self.obfuscated = Some(obfuscated);
    }

    fn font(&mut self, font: Option<String>) {
        self.font = font;
    }

    fn insertion(&mut self, insertion: Option<String>) {
        self.insertion = insertion;
    }

    fn click_event(&mut self, click_event: Option<ClickEvent>) {
        self.click_event = click_event;
    }

    fn hover_event(&mut self, hover_event: Option<HoverEvent>) {
        self.hover_event = hover_event;
    }

    fn get_color(&self) -> Option<&ChatColor> {
        self.color.as_ref()
    }

    fn get_bold(&self) -> Option<bool> {
        self.bold
    }

    fn get_italic(&self) -> Option<bool> {
        self.italic
    }

    fn get_underlined(&self) -> Option<bool> {
        self.underlined
    }

    fn get_strikethrough(&self) -> Option<bool> {
        self.strikethrough
    }

    fn get_obfuscated(&self) -> Option<bool> {
        self.obfuscated
    }

    fn get_font(&self) -> Option<&String> {
        if self.version >= 713 {
            self.font.as_ref()
        } else {
            None
        }
    }

    fn get_insertion(&self) -> Option<&String> {
        if self.version >= 5 {
            self.insertion.as_ref()
        } else {
            None
        }
    }

    fn get_click_event(&self) -> Option<&ClickEvent> {
        self.click_event.as_ref()
    }

    fn get_hover_event(&self) -> Option<&HoverEvent> {
        self.hover_event.as_ref()
    }
}
