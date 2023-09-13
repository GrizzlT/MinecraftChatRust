#![cfg(feature = "macros")]
use std::assert_eq;

use mc_chat::{chat, Chat, TextColor};

#[test]
fn plaintext() {
    let orig_chat = Chat::text("Hello world!!");
    let chat = chat!("Hello world!!");
    assert_eq!(orig_chat, chat);
}

#[test]
fn single_color() {
    let orig_chat = Chat::text("Hello world!!").color(TextColor::Gold);
    let chat = chat!("§6Hello world!!");
    assert_eq!(orig_chat, chat);
}

#[test]
fn mixed() {
    let orig_chat = Chat::text("Testing ")
        .color(TextColor::DarkRed)
        .bold(true)
        .obfuscated(true)
        .child(
            Chat::text("")
                .bold(true)
                .child(Chat::text("overly ").color(TextColor::DarkGreen))
                .child(Chat::text("much").color(TextColor::Gray)),
        );
    let chat = chat!("§2§3§4§l§kTesting §l§l§2overly §7much");
    assert_eq!(orig_chat, chat);
}

#[test]
fn variable_sub() {
    let variable = String::from("My Variableee");
    let orig_chat = Chat::text("Var: ").color(TextColor::Blue).child(
        Chat::text(variable.clone())
            .color(TextColor::Purple)
            .underlined(true)
            .child(Chat::text("!!").color(TextColor::Yellow)),
    );
    let chat = chat!("§9Var: §5§n", variable, "§e!!");
    assert_eq!(orig_chat, chat);
}

#[test]
fn custom_delimiter() {
    let orig_chat = Chat::text("")
        .child(Chat::text("Hello §").color(TextColor::Black))
        .child(Chat::text("world!!").color(TextColor::White));
    let chat = chat!("§@" => "§@0Hello §§@fworld!!");
    assert_eq!(orig_chat, chat);
}
