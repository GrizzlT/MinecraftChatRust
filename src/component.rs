use crate::{ChatColor, ClickEvent, HoverEvent};
#[cfg(feature = "use-serde")]
use serde::Serialize;

#[cfg(feature = "use-serde")]
use erased_serde::serialize_trait_object;

///
/// Represents a text object from minecraft that
/// can be serialized and deserialized into a JSON-message
/// suitable for sending across the minecraft protocol.
/// (see [Wiki.vg](https://wiki.vg/Chat))
#[cfg(feature = "use-serde")]
#[cfg_attr(not(feature = "use-serde"), doc(hidden))]
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
#[cfg(not(feature = "use-serde"))]
#[cfg_attr(feature = "use-serde", doc(hidden))]
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

#[cfg(feature = "use-serde")]
serialize_trait_object!(Component);

/// The central struct containing all style information about a [`Component`].
#[cfg_attr(feature = "use-serde", derive(Serialize))]
pub struct ComponentStyle {
    #[cfg_attr(feature = "use-serde", serde(skip_serializing_if = "Option::is_none"))]
    pub bold: Option<bool>,
    #[cfg_attr(feature = "use-serde", serde(skip_serializing_if = "Option::is_none"))]
    pub italic: Option<bool>,
    #[cfg_attr(feature = "use-serde", serde(skip_serializing_if = "Option::is_none"))]
    pub underlined: Option<bool>,
    #[cfg_attr(feature = "use-serde", serde(skip_serializing_if = "Option::is_none"))]
    pub strikethrough: Option<bool>,
    #[cfg_attr(feature = "use-serde", serde(skip_serializing_if = "Option::is_none"))]
    pub obfuscated: Option<bool>,
    #[cfg_attr(feature = "use-serde", serde(skip_serializing_if = "Option::is_none"))]
    pub color: Option<ChatColor>,
    /// # Warning
    /// This is only available since 1.16.
    ///
    /// Implementations of serializers for older versions should ignore this field at all times.
    #[cfg_attr(feature = "use-serde", serde(skip_serializing_if = "Option::is_none"))]
    pub font: Option<String>,
    /// # Warning
    /// This is not available before 1.8.
    ///
    /// Implementations of serializers for older versions should ignore this field at all times.
    #[cfg_attr(feature = "use-serde", serde(skip_serializing_if = "Option::is_none"))]
    pub insertion: Option<String>,
    #[cfg_attr(feature = "use-serde", serde(rename = "clickEvent", skip_serializing_if = "Option::is_none"))]
    pub click_event: Option<ClickEvent>,
    #[cfg_attr(feature = "use-serde", serde(rename = "hoverEvent", skip_serializing_if = "Option::is_none"))]
    pub hover_event: Option<HoverEvent>
}

impl ComponentStyle {
    /// Returns a new instance with all values set to [`None`].
    pub fn new() -> ComponentStyle {
        ComponentStyle {
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            color: None,
            font: None,
            insertion: None,
            click_event: None,
            hover_event: None
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
        self.font = style.font.clone();
        self.insertion = style.insertion.clone();
    }

    /// Assigns all [`None`] fields except [`ComponentStyle::hover_event`] and [`ComponentStyle::click_event`]
    /// to the values of the specified style.
    pub fn merge_style(&mut self, style: &ComponentStyle) {
        if self.bold.is_none() { self.bold = style.bold; }
        if self.italic.is_none() { self.italic = style.italic; }
        if self.underlined.is_none() { self.underlined = style.underlined; }
        if self.strikethrough.is_none() { self.strikethrough = style.strikethrough; }
        if self.obfuscated.is_none() { self.obfuscated = style.obfuscated; }
        if self.color.is_none() { self.color = style.color.clone(); }
        if self.font.is_none() { self.font = style.font.clone(); }
        if self.insertion.is_none() { self.insertion = style.insertion.clone() }
    }

    /// Resets all fields to default (being [`None`]).
    pub fn reset(&mut self) {
        self.bold = None;
        self.italic = None;
        self.underlined = None;
        self.strikethrough = None;
        self.obfuscated = None;
        self.color = None;
        self.font = None;
        self.insertion = None;
        self.hover_event = None;
        self.click_event = None;
    }
}