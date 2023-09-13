#![doc = include_str!("../README.md")]

use mc_chat_core::chat_core;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro]
pub fn chat(input: TokenStream) -> TokenStream { chat_core(input.into()).into() }

