mod component;
mod style;

mod tests;

pub use component::{
    ChatComponent, ComponentType, KeybindComponent, ScoreComponent, SelectorComponent,
    TextComponent, TranslationComponent,
};
pub use style::{
    ChatColor, ClickEvent, ComponentStyle, HoverEvent, VERSION_1_15, VERSION_1_16, VERSION_1_7,
    VERSION_1_8,
};
