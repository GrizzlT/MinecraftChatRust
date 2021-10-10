#![cfg(test)]

use crate::{ChatColor, ClickEvent, DecorateComponent, HoverEvent};
use crate::text::TextComponent;

#[test]
fn test_lifetimes() {
    let test_obj = TextComponent::from_text("Test String")
        .color_if_absent(ChatColor::Custom(String::from("test_color")))
        .click_event(Some(ClickEvent::SuggestCommand(String::from("/msg Herobrine "))))
        .hover_event(Some(HoverEvent::ShowText(Box::new(TextComponent::from_text("My second text!!!!")
            .color_if_absent(ChatColor::Gold)))));

    let output = serde_json::to_string(&test_obj).unwrap();

    println!("test_obj: {}", output);
}