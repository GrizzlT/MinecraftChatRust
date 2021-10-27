#![cfg(test)]

use crate::{ChatColor, ComponentStyle, DecorateComponent};
use crate::{ClickEvent, TextComponent};

#[test]
fn test_lifetimes() {
    let test_obj = TextComponent::from_text("Test String", ComponentStyle::v1_7())
        .color_if_absent(ChatColor::Custom(String::from("test_color")))
        .bold(true)
        .font(Some("test".to_string()))
        .click_event(Some(ClickEvent::CopyToClipBoard(
            "test copy to clipboard".to_string(),
        )))
        .insertion(Some("Big Insertion!".to_string()));

    let output = serde_json::to_string(&test_obj).unwrap();

    println!("test_text: {}", output);

    assert_eq!(output, "{\"text\":\"Test String\",\"bold\":true}");
}
