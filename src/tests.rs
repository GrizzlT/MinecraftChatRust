#![cfg(test)]

use crate::{ChatColor, DecorateComponent};
use crate::text::TextComponent;

#[test]
fn test_lifetimes() {
    let test_obj = TextComponent::from_text(String::from("Test String"))
        .color_if_absent(ChatColor::Blue);

    let output = serde_json::to_string(&test_obj)?;

    println!("test_obj: {}", output);
}