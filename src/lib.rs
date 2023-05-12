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

mod component;
mod style;
mod freeze;

mod tests;

pub use component::{
    Chat, Component, KeybindComponent, ScoreComponent, SelectorComponent,
    TextComponent, TranslationComponent,
};
pub use style::{ChatColor, ClickEvent, Style, HoverEvent};

/// The version number of the Minecraft protocol for 1.7
pub const VERSION_1_7: u32 = 4;
/// The version number of the Minecraft protocol for 1.8
pub const VERSION_1_8: u32 = 47;
/// The version number of the Minecraft protocol for 1.15
pub const VERSION_1_15: u32 = 573;
/// The version number of the Minecraft protocol for 1.16
pub const VERSION_1_16: u32 = 735;
