#![cfg(test)]

#[cfg(feature = "serde")]
mod serde_support {
    use std::str::FromStr;

    use serde_json::Value;
    use uuid::Uuid;

    use crate::{Chat, VERSION_1_8, ClickEvent, HoverEvent, EntityTooltip, TranslationComponent};

    #[test]
    pub fn chat_serialize() {
        let chat_orig = Chat::text("Sample text");
        let serialized_str = chat_orig.serialize_str(VERSION_1_8).unwrap();
        assert_eq!("{\"text\":\"Sample text\"}", serialized_str);

        let value = Value::from_str("{\"text\":\"Sample text\"}").unwrap();
        let chat: Chat = serde_json::from_value(value).unwrap();
        assert_eq!(chat_orig, chat);
    }

    #[test]
    pub fn standard_chat() {
        let chat_orig = Chat::component(TranslationComponent::new("chat.type.text")
            .argument(Chat::text("Herobrine").insertion(Some("Herobrine"))
                .click(Some(ClickEvent::suggest("/msg Herobrine ")))
                .hover(Some(HoverEvent::ShowEntity(EntityTooltip::new(Some(Chat::text("Herobrine")), Option::<&str>::None, Some(Uuid::from_str("f84c6a79-0a4e-45e0-879b-cd49ebd4c4e2").unwrap()))))))
            .argument(Chat::text("I don't exist")));

        let serialized_str = r#"{"translate":"chat.type.text","with":[{"text":"Herobrine","clickEvent":{"action":"suggest_command","value":"/msg Herobrine "},"hoverEvent":{"action":"show_entity","value":"{id:f84c6a79-0a4e-45e0-879b-cd49ebd4c4e2,name:Herobrine}"},"insertion":"Herobrine"},{"text":"I don't exist"}]}"#;
        let chat: Chat = serde_json::from_str(serialized_str).unwrap();
        assert_eq!(chat_orig, chat);
    }
}
