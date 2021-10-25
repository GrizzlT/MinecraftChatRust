macro_rules! generate_component_style_impl_1_7 {
    () => {
        impl ComponentStyleEditable for ComponentStyle {
            #[inline]
            fn color(&mut self, color: Option<ChatColor>) {
                self.color = color
            }

            #[inline]
            fn color_absent(&self) -> bool {
                self.color.is_none()
            }

            #[inline]
            fn color_if_absent(&mut self, color: ChatColor) {
                if self.color.is_none() {
                    self.color = Some(color)
                }
            }

            #[inline]
            fn bold(&mut self, bold: bool) {
                self.bold = if bold { Some(true) } else { None }
            }

            #[inline]
            fn italic(&mut self, italic: bool) {
                self.italic = if italic { Some(true) } else { None }
            }

            #[inline]
            fn underlined(&mut self, underlined: bool) {
                self.underlined = if underlined { Some(true) } else { None }
            }

            #[inline]
            fn strikethrough(&mut self, strikethrough: bool) {
                self.strikethrough = if strikethrough { Some(true) } else { None }
            }

            #[inline]
            fn obfuscated(&mut self, obfuscated: bool) {
                self.obfuscated = if obfuscated { Some(true) } else { None }
            }

            #[inline]
            fn font(&mut self, _font: Option<String>) {
                // not implemented
            }

            #[inline]
            fn insertion(&mut self, _insertion: Option<String>) {
                // not implemented
            }

            #[inline]
            fn click_event(&mut self, click_event: Option<ClickEvent>) {
                self.click_event = click_event
            }

            #[inline]
            fn hover_event(&mut self, hover_event: Option<HoverEvent>) {
                self.hover_event = hover_event
            }
        }
    }
}

macro_rules! generate_component_style_impl_1_8 {
    () => {
        impl ComponentStyleEditable for ComponentStyle {
            #[inline]
            fn color(&mut self, color: Option<ChatColor>) {
                self.default.color(color)
            }

            #[inline]
            fn color_absent(&self) -> bool {
                self.default.color_absent()
            }

            #[inline]
            fn color_if_absent(&mut self, color: ChatColor) {
                self.default.color_if_absent(color)
            }

            #[inline]
            fn bold(&mut self, bold: bool) {
                self.default.bold(bold)
            }

            #[inline]
            fn italic(&mut self, italic: bool) {
                self.default.italic(italic)
            }

            #[inline]
            fn underlined(&mut self, underlined: bool) {
                self.default.underlined(underlined)
            }

            #[inline]
            fn strikethrough(&mut self, strikethrough: bool) {
                self.default.strikethrough(strikethrough)
            }

            #[inline]
            fn obfuscated(&mut self, obfuscated: bool) {
                self.default.obfuscated(obfuscated)
            }

            #[inline]
            fn font(&mut self, _font: Option<String>) {
                // not implemented
            }

            #[inline]
            fn insertion(&mut self, insertion: Option<String>) {
                self.insertion = insertion
            }

            #[inline]
            fn click_event(&mut self, click_event: Option<ClickEvent>) {
                self.default.click_event(click_event)
            }

            #[inline]
            fn hover_event(&mut self, hover_event: Option<HoverEvent>) {
                self.default.hover_event(hover_event)
            }
        }
    }
}

macro_rules! generate_component_style_impl_1_16 {
    () => {
        impl ComponentStyleEditable for ComponentStyle {
            #[inline]
            fn color(&mut self, color: Option<ChatColor>) {
                self.default.color(color)
            }

            #[inline]
            fn color_absent(&self) -> bool {
                self.default.color_absent()
            }

            #[inline]
            fn color_if_absent(&mut self, color: ChatColor) {
                self.default.color_if_absent(color)
            }

            #[inline]
            fn bold(&mut self, bold: bool) {
                self.default.bold(bold)
            }

            #[inline]
            fn italic(&mut self, italic: bool) {
                self.default.italic(italic)
            }

            #[inline]
            fn underlined(&mut self, underlined: bool) {
                self.default.underlined(underlined)
            }

            #[inline]
            fn strikethrough(&mut self, strikethrough: bool) {
                self.default.strikethrough(strikethrough)
            }

            #[inline]
            fn obfuscated(&mut self, obfuscated: bool) {
                self.default.obfuscated(obfuscated)
            }

            #[inline]
            fn font(&mut self, font: Option<String>) {
                self.font = font
            }

            #[inline]
            fn insertion(&mut self, insertion: Option<String>) {
                self.default.insertion(insertion)
            }

            #[inline]
            fn click_event(&mut self, click_event: Option<ClickEvent>) {
                self.default.click_event(click_event)
            }

            #[inline]
            fn hover_event(&mut self, hover_event: Option<HoverEvent>) {
                self.default.hover_event(hover_event)
            }
        }
    }
}

macro_rules! generate_component_style_impl_overall {
    () => {
        impl ComponentStyleEditable for ComponentStyle {
            fn color(&mut self, color: Option<ChatColor>) {
                match self {
                    ComponentStyle::V1_7(style) => style.color(color),
                    ComponentStyle::V1_8(style) => style.color(color),
                    ComponentStyle::V1_16(style) => style.color(color),
                }
            }

            fn color_absent(&self) -> bool {
                match self {
                    ComponentStyle::V1_7(style) => style.color_absent(),
                    ComponentStyle::V1_8(style) => style.color_absent(),
                    ComponentStyle::V1_16(style) => style.color_absent(),
                }
            }

            fn color_if_absent(&mut self, color: ChatColor) {
                match self {
                    ComponentStyle::V1_7(style) => style.color_if_absent(color),
                    ComponentStyle::V1_8(style) => style.color_if_absent(color),
                    ComponentStyle::V1_16(style) => style.color_if_absent(color),
                }
            }

            fn bold(&mut self, bold: bool) {
                match self {
                    ComponentStyle::V1_7(style) => style.bold(bold),
                    ComponentStyle::V1_8(style) => style.bold(bold),
                    ComponentStyle::V1_16(style) => style.bold(bold),
                }
            }

            fn italic(&mut self, italic: bool) {
                match self {
                    ComponentStyle::V1_7(style) => style.italic(italic),
                    ComponentStyle::V1_8(style) => style.italic(italic),
                    ComponentStyle::V1_16(style) => style.italic(italic),
                }
            }

            fn underlined(&mut self, underlined: bool) {
                match self {
                    ComponentStyle::V1_7(style) => style.underlined(underlined),
                    ComponentStyle::V1_8(style) => style.underlined(underlined),
                    ComponentStyle::V1_16(style) => style.underlined(underlined),
                }
            }

            fn strikethrough(&mut self, strikethrough: bool) {
                match self {
                    ComponentStyle::V1_7(style) => style.strikethrough(strikethrough),
                    ComponentStyle::V1_8(style) => style.strikethrough(strikethrough),
                    ComponentStyle::V1_16(style) => style.strikethrough(strikethrough),
                }
            }

            fn obfuscated(&mut self, obfuscated: bool) {
                match self {
                    ComponentStyle::V1_7(style) => style.obfuscated(obfuscated),
                    ComponentStyle::V1_8(style) => style.obfuscated(obfuscated),
                    ComponentStyle::V1_16(style) => style.obfuscated(obfuscated),
                }
            }

            fn font(&mut self, font: Option<String>) {
                match self {
                    ComponentStyle::V1_7(style) => style.font(font),
                    ComponentStyle::V1_8(style) => style.font(font),
                    ComponentStyle::V1_16(style) => style.font(font),
                }
            }

            fn insertion(&mut self, insertion: Option<String>) {
                match self {
                    ComponentStyle::V1_7(style) => style.insertion(insertion),
                    ComponentStyle::V1_8(style) => style.insertion(insertion),
                    ComponentStyle::V1_16(style) => style.insertion(insertion),
                }
            }

            fn click_event(&mut self, click_event: Option<ClickEvent>) {
                match self {
                    ComponentStyle::V1_7(style) => style.click_event(click_event),
                    ComponentStyle::V1_8(style) => style.click_event(click_event),
                    ComponentStyle::V1_16(style) => style.click_event(click_event),
                }
            }

            fn hover_event(&mut self, hover_event: Option<HoverEvent>) {
                match self {
                    ComponentStyle::V1_7(style) => style.hover_event(hover_event),
                    ComponentStyle::V1_8(style) => style.hover_event(hover_event),
                    ComponentStyle::V1_16(style) => style.hover_event(hover_event),
                }
            }
        }
    }
}