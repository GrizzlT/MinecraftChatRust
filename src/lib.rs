//!
//! This crate provides Rust objects that map to the minecraft
//! protocol raw JSON message format used for chat messages, books, titles...
//!
//! Serialization and Deserialization is implemented using serde. We plan on implementing legacy text soon!
//!
//! Please check out our [github](https://github.com/GrizzlT/MinecraftChatRust) and
//! feel free to contribute.

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
