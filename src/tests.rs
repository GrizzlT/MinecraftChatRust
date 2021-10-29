#![cfg(test)]

use crate::component::{ChatComponent};
use crate::style::{ChatColor, ClickEvent, ComponentStyle};

#[test]
fn test_lifetimes() {
    let mut test_obj = ChatComponent::from_text("Test String", ComponentStyle::v1_16()
        .color_if_absent(ChatColor::custom("test_color"))
        .font(Some("test"))
        .click_event(Some(ClickEvent::clipboard("test for copy to clipboard!")))
        .insertion(Some("Testing insertion haha!"))
        .bold(true));

    test_obj.change_version(crate::style::VERSION_1_7);

    let output = serde_json::to_string(&test_obj).unwrap();

    println!("test_text: {}", output);

    assert_eq!(output, "{\"text\":\"Test String\",\"bold\":true}");
}
