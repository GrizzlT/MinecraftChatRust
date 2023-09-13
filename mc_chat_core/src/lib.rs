use proc_macro2::TokenStream;
use proc_macro_error::{abort, set_dummy};
use quote::quote;

use crate::parsing::{map_to_tree, LegacyChat};

mod parsing;

pub fn chat_core(input: TokenStream) -> TokenStream {
    set_dummy(quote!(unimplemented!(
        "Compile time error in chat!() macro"
    )));

    let legacy_chat: LegacyChat = match syn::parse2(input) {
        Ok(parts) => parts,
        Err(error) => abort!(error.span(), error.to_string()),
    };

    let root = match map_to_tree(legacy_chat) {
        Ok(root) => root,
        Err(error) => abort!(error.span(), error.to_string()),
    };

    quote!(#root)
}
