#![cfg(test)]

#[cfg(feature = "serde")]
mod serde_support {
    use std::str::FromStr;

    use serde_json::Value;

    use crate::{Chat, VERSION_1_8};

    #[test]
    pub fn chat_serialize() {
        let chat_orig = Chat::text("Sample text");
        let serialized_str = chat_orig.serialize_str(VERSION_1_8).unwrap();
        assert_eq!("{\"text\":\"Sample text\"}", serialized_str);

        let value = Value::from_str("{\"text\":\"Sample text\"}").unwrap();
        let chat: Chat = serde_json::from_value(value).unwrap();
        assert_eq!(chat_orig, chat);
    }
}
