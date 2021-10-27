#![cfg(test)]

use crate::{ChatColor, Component, ComponentStyle, ComponentStyleEditable, DecorateComponent};
use crate::{ClickEvent, TextComponent};

#[test]
fn test_lifetimes() {
    let mut test_obj = TextComponent::from_text("Test String", ComponentStyle::v1_16())
        .color_if_absent(ChatColor::Custom(String::from("test_color")))
        .bold(true)
        .font(Some("test".to_string()))
        .click_event(Some(ClickEvent::CopyToClipBoard(
            "test copy to clipboard".to_string(),
        )))
        .insertion(Some("Big Insertion!".to_string()));

    test_obj.get_style_mut().change_version(crate::VERSION_1_7);

    let output = serde_json::to_string(&test_obj).unwrap();

    println!("test_text: {}", output);

    assert_eq!(output, "{\"text\":\"Test String\",\"bold\":true}");
}
