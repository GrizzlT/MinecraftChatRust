//!
//! This crate provides Rust objects that map to the minecraft
//! protocol raw JSON message format used for chat messages, books, titles...
//!
//! ### Serialization/Deserialization
//!
//! Serialization and Deserialization is implemented using serde. Excluding a
//! few rare cases (mainly the `storage` component and the `separator` field for [`SelectorComponent`]),
//! all serialization and deserialization should happen correctly.
//!
//! We plan on implementing legacy text soon!
//!
//! ### Contributing
//!
//! Please check out our [github](https://github.com/GrizzlT/MinecraftChatRust) and
//! feel free to contribute.
//!
//! This crate would need more testing, all help appreciated!!

mod component;
mod style;
mod freeze;

mod tests;

pub use component::{
    ChatComponent, ComponentType, KeybindComponent, ScoreComponent, SelectorComponent,
    TextComponent, TranslationComponent,
};
pub use style::{
    ChatColor, ClickEvent, ComponentStyle, HoverEvent, VERSION_1_15, VERSION_1_16, VERSION_1_7,
    VERSION_1_8,
};
