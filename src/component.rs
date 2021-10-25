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
pub enum ComponentStyle {
    V1_7(v1_7::ComponentStyle),
    V1_8(v1_8::ComponentStyle),
    V1_16(v1_16::ComponentStyle),
}

pub trait ComponentStyleEditable {
    fn color(&mut self, color: Option<ChatColor>);

    fn color_absent(&self) -> bool;

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
}

pub mod v1_7 {
    use crate::{ChatColor, ClickEvent, HoverEvent};
    use crate::component::ComponentStyleEditable;

    #[cfg_attr(feature = "serde-support", derive(serde::Serialize))]
    pub struct ComponentStyle {
        #[cfg_attr(feature = "serde-support", serde(skip_serializing_if = "Option::is_none"))]
        pub bold: Option<bool>,
        #[cfg_attr(feature = "serde-support", serde(skip_serializing_if = "Option::is_none"))]
        pub italic: Option<bool>,
        #[cfg_attr(feature = "serde-support", serde(skip_serializing_if = "Option::is_none"))]
        pub underlined: Option<bool>,
        #[cfg_attr(feature = "serde-support", serde(skip_serializing_if = "Option::is_none"))]
        pub strikethrough: Option<bool>,
        #[cfg_attr(feature = "serde-support", serde(skip_serializing_if = "Option::is_none"))]
        pub obfuscated: Option<bool>,
        #[cfg_attr(feature = "serde-support", serde(skip_serializing_if = "Option::is_none"))]
        pub color: Option<crate::ChatColor>,
        #[cfg_attr(feature = "serde-support", serde(rename = "clickEvent", skip_serializing_if = "Option::is_none"))]
        pub click_event: Option<crate::ClickEvent>,
        #[cfg_attr(feature = "serde-support", serde(rename = "hoverEvent", skip_serializing_if = "Option::is_none"))]
        pub hover_event: Option<crate::HoverEvent>
    }

    impl ComponentStyle {
        #[inline]
        pub fn new() -> ComponentStyle {
            ComponentStyle {
                bold: None,
                italic: None,
                underlined: None,
                strikethrough: None,
                obfuscated: None,
                color: None,
                click_event: None,
                hover_event: None
            }
        }

        pub(crate) fn apply_style_self(&mut self, style: &ComponentStyle) {
            self.bold = style.bold;
            self.italic = style.italic;
            self.underlined = style.underlined;
            self.strikethrough = style.strikethrough;
            self.obfuscated = style.obfuscated;
            self.color = style.color.clone();
        }

        pub(crate) fn apply_style(&mut self, style: &super::ComponentStyle) {
            match style {
                super::ComponentStyle::V1_7(style) => self.apply_style_self(style),
                super::ComponentStyle::V1_8(style) => self.apply_style_self(&style.default),
                super::ComponentStyle::V1_16(style) => self.apply_style_self(&style.default.default),
            }
        }

        pub(crate) fn reset(&mut self) {
            self.bold = None;
            self.italic = None;
            self.underlined = None;
            self.strikethrough = None;
            self.obfuscated = None;
            self.color = None;
            self.hover_event = None;
            self.click_event = None;
        }
    }

    generate_component_style_impl_1_7!();
}

pub mod v1_8 {
    use crate::{ChatColor, ClickEvent, HoverEvent};
    use crate::component::ComponentStyleEditable;

    #[cfg_attr(feature = "serde-support", derive(serde::Serialize))]
    pub struct ComponentStyle {
        #[cfg_attr(feature = "serde-support", serde(flatten))]
        pub default: super::v1_7::ComponentStyle,
        #[cfg_attr(feature = "serde-support", serde(skip_serializing_if = "Option::is_none"))]
        pub insertion: Option<String>,
    }

    impl ComponentStyle {
        pub fn new() -> ComponentStyle {
            ComponentStyle {
                default: super::v1_7::ComponentStyle::new(),
                insertion: None
            }
        }

        pub(crate) fn apply_style_self(&mut self, style: &ComponentStyle) {
            self.default.apply_style_self(&style.default);
            self.insertion = style.insertion.clone();
        }

        pub(crate) fn apply_style(&mut self, style: &super::ComponentStyle) {
            match style {
                super::ComponentStyle::V1_7(style) => self.default.apply_style_self(style),
                super::ComponentStyle::V1_8(style) => self.apply_style_self(style),
                super::ComponentStyle::V1_16(style) => self.apply_style_self(&style.default),
            }
        }

        pub(crate) fn reset(&mut self) {
            self.default.reset();
            self.insertion = None;
        }
    }

    generate_component_style_impl_1_8!();
}

pub mod v1_16 {
    use crate::{ChatColor, ClickEvent, HoverEvent};
    use crate::component::ComponentStyleEditable;

    #[cfg_attr(feature = "serde-support", derive(serde::Serialize))]
    pub struct ComponentStyle {
        #[cfg_attr(feature = "serde-support", serde(flatten))]
        pub default: super::v1_8::ComponentStyle,
        #[cfg_attr(feature = "serde-support", serde(skip_serializing_if = "Option::is_none"))]
        pub font: Option<String>,
    }

    impl ComponentStyle {
        pub fn new() -> ComponentStyle {
            ComponentStyle {
                default: super::v1_8::ComponentStyle::new(),
                font: None
            }
        }

        pub(crate) fn apply_style_self(&mut self, style: &ComponentStyle) {
            self.default.apply_style_self(&style.default);
            self.font = style.font.clone();
        }

        pub(crate) fn apply_style(&mut self, style: &super::ComponentStyle) {
            match style {
                super::ComponentStyle::V1_7(style) => self.default.default.apply_style_self(style),
                super::ComponentStyle::V1_8(style) => self.default.apply_style_self(style),
                super::ComponentStyle::V1_16(style) => self.apply_style_self(style),
            }
        }

        pub(crate) fn reset(&mut self) {
            self.default.reset();
            self.font = None;
        }
    }

    generate_component_style_impl_1_16!();
}

impl ComponentStyle {
    pub fn v1_7() -> ComponentStyle {
        ComponentStyle::V1_7(v1_7::ComponentStyle::new())
    }

    pub fn v1_8() -> ComponentStyle {
        ComponentStyle::V1_8(v1_8::ComponentStyle::new())
    }

    pub fn v1_16() -> ComponentStyle {
        ComponentStyle::V1_16(v1_16::ComponentStyle::new())
    }

    /// Assigns all fields except [`ComponentStyle::hover_event`] and [`ComponentStyle::click_event`]
    /// to the values of the specified style.
    pub fn apply_style(&mut self, component_style: &ComponentStyle) {
        match self {
            ComponentStyle::V1_7(style) => style.apply_style(component_style),
            ComponentStyle::V1_8(style) => style.apply_style(component_style),
            ComponentStyle::V1_16(style) => style.apply_style(component_style)
        }
    }

    /// Resets all fields to default (being [`None`]).
    pub fn reset(&mut self) {
        match self {
            ComponentStyle::V1_7(style) => style.reset(),
            ComponentStyle::V1_8(style) => style.reset(),
            ComponentStyle::V1_16(style) => style.reset(),
        }
    }
}

generate_component_style_impl_overall!();