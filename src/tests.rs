#![cfg(test)]

use crate::component::ChatComponent;
use crate::style::{ChatColor, ClickEvent, ComponentStyle};

use serde_json::json;

#[test]
fn test_serializing() {
    let mut test_obj = ChatComponent::from_text(
        "Test String",
        ComponentStyle::v1_16()
            .color_if_absent(ChatColor::custom("test_color"))
            .font(Some("test"))
            .click_event(Some(ClickEvent::clipboard("test for copy to clipboard!")))
            .insertion(Some("Testing insertion haha!"))
            .bold(true),
    );

    test_obj.change_version(crate::style::VERSION_1_7);

    let output = serde_json::to_string(&test_obj).unwrap();

    println!("test_text: {}", output);

    assert_eq!(output, "{\"text\":\"Test String\",\"bold\":true}");
}

#[test]
fn test_deserializing() {
    let obj: ChatComponent = serde_json::from_str(r#"{"translate":"chat.type.text","with":[{"text":"Herobrine","clickEvent":{"action":"suggest_command","value":"/msg Herobrine "},"hoverEvent":{"action":"show_entity","value":"{id:f84c6a79-0a4e-45e0-879b-cd49ebd4c4e2,name:Herobrine}"},"insertion":"Herobrine"},{"text":"I don't exist"}]}"#).unwrap();

    assert_eq!(
        serde_json::to_value(&obj).unwrap(),
        json!({"translate":"chat.type.text","with":[{"text":"Herobrine","clickEvent":{"action":"suggest_command","value":"/msg Herobrine "},"hoverEvent":{"action":"show_entity","value":"{id:f84c6a79-0a4e-45e0-879b-cd49ebd4c4e2,name:Herobrine}"},"insertion":"Herobrine"},{"text":"I don't exist"}]})
    );

    println!("test: {:?}", obj);
}
