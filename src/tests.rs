#![cfg(test)]

use crate::{ChatColor, ComponentStyle, DecorateComponent};
use crate::TextComponent;

#[test]
fn test_lifetimes() {
    let test_obj = TextComponent::from_text("Test String", ComponentStyle::v1_8())
        .color_if_absent(ChatColor::Custom(String::from("test_color")))
        .bold(true)
        .font(Some("test".to_string()));

    let output = serde_json::to_string(&test_obj).unwrap();

    println!("test_text: {}", output);
}