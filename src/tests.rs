#![cfg(test)]

use crate::{ChatColor, ClickEvent, Component, DecorateComponent, HoverEvent};
use crate::{TextComponent, TranslatableComponent};

#[test]
fn test_lifetimes() {
    let mut test_obj = TextComponent::from_text("Test String")
        .color_if_absent(ChatColor::Custom(String::from("test_color")))
        .bold(true)
        .click_event(Some(ClickEvent::SuggestCommand(String::from("/msg Herobrine "))))
        .hover_event(Some(HoverEvent::ShowText(Box::new(TextComponent::from_text("My second text!!!!")
            .color_if_absent(ChatColor::Gold)))));

    test_obj.append(Box::new(TextComponent::from_text("Special text").bold(true).italic(true).color(Some(ChatColor::Red))));

    let translatable = TranslatableComponent::from_key("chat.type.text")
        .add_arg(Box::new(TextComponent::from_text("heheheh")))
        .strikethrough(true)
        .underlined(true);

    let output = serde_json::to_string(&test_obj).unwrap();
    let output2 = serde_json::to_string(&translatable).unwrap();

    println!("test_text: {}", output);
    println!("translatable: {}", output2);
}