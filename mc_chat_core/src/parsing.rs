use std::fmt::Debug;
use std::collections::HashSet;

use proc_macro2::{TokenStream, Span};
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::{parse::Parse, LitStr, Token, Expr, punctuated::Punctuated, ExprPath, parse_quote};

pub fn map_to_tree(legacy_chat: LegacyChat) -> syn::Result<ExpandedChatPart> {
    // Root Chat component
    let mut root = ExpandedChatPart::new(quote!(::mc_chat::Chat::text("")));

    let mut current_parent = ExpandedChatPart::default();
    for part in legacy_chat.chat_parts.into_iter().rev() {
        match part {
            ChatPart::Literal(part) => {
                let pattern = legacy_chat.pattern.value();
                let value = part.value();
                let mut piece_iter = value.rsplit(&pattern);
                let mut piece = piece_iter.next().ok_or(syn::Error::new(part.span(), "Empty string should be able to be rsplit-ted"))?;
                let mut next_piece = piece_iter.next();
                loop {
                    if piece.is_empty() {
                        if next_piece.is_some() {
                            abort!(part.span(), "Invalid escape sequence detected!");
                        }
                    } else if next_piece.is_none() {
                        current_parent.children.push(ExpandedChatPart::new(quote!(::mc_chat::Chat::text(#piece))))
                    } else {
                        let mut chars = piece.chars();
                        let code = chars.next();
                        if code.is_none() || !"0123456789abcdefklmnor".contains(code.unwrap()) {
                            abort!(part.span(), "Invalid escape sequence detected!");
                        }
                        let code = code.unwrap();
                        let rest = chars.as_str();

                        if "0123456789abcdef".contains(code) {
                            if current_parent.is_placeholder() {
                                if rest != "" {
                                    let mut node = ExpandedChatPart::new(quote!(::mc_chat::Chat::text(#rest)));
                                    node.color = Some(color_from_code(part.span(), code)?);
                                    current_parent.children.push(node);
                                }
                            } else {
                                if rest != "" {
                                    let mut node = ExpandedChatPart::new(quote!(::mc_chat::Chat::text(#rest)));
                                    node.color = Some(color_from_code(part.span(), code)?);
                                    // reverse for correct left to right order
                                    current_parent.children.reverse();
                                    node.children.push(current_parent);
                                    current_parent = ExpandedChatPart::default();
                                    current_parent.children.push(node);
                                } else {
                                    if current_parent.color.is_none() {
                                        current_parent.color = Some(color_from_code(part.span(), code)?);
                                    }
                                }
                            }
                        } else {
                            if code == 'r' {
                                if current_parent.is_placeholder() {
                                    root.children.extend(current_parent.children);
                                } else {
                                    current_parent.children.reverse();
                                    root.children.push(current_parent);
                                }
                                current_parent = ExpandedChatPart::default();
                                if rest != "" {
                                    root.children.push(ExpandedChatPart::new(quote!(::mc_chat::Chat::text(#rest))));
                                }
                            } else {
                                if rest != "" {
                                    let mut node = ExpandedChatPart::new(quote!(::mc_chat::Chat::text(#rest)));
                                    node.extra_style.insert(code);
                                    if current_parent.is_placeholder() {
                                        node.children.extend(current_parent.children);
                                    } else {
                                        current_parent.children.reverse();
                                        node.children.push(current_parent);
                                    }
                                    current_parent = node;
                                } else {
                                    if !current_parent.is_placeholder() || !current_parent.children.is_empty() {
                                        current_parent.extra_style.insert(code);
                                        if current_parent.tokens.is_none() {
                                            current_parent.tokens = Some(quote!(::mc_chat::Chat::text("")));
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if next_piece.is_none() {
                        break;
                    }
                    piece = next_piece.unwrap();
                    next_piece = piece_iter.next();
                }
            }
            ChatPart::Variable(part) => {
                let mut node = ExpandedChatPart::new(quote!(::mc_chat::Chat::text(#part)));
                if current_parent.is_placeholder() {
                    node.children.extend(current_parent.children);
                } else {
                    node.children.push(current_parent);
                }
                current_parent = node;
            }
        }
    }
    if current_parent.is_placeholder() {
        root.children.extend(current_parent.children);
    } else {
        root.children.push(current_parent);
    }
    if root.children.len() == 1 {
        Ok(root.children.remove(0))
    } else {
        root.children.reverse();
        Ok(root)
    }
}

pub fn color_from_code(span: Span, code: char) -> syn::Result<ExprPath> {
    Ok(match code {
        '0' => parse_quote!(::mc_chat::TextColor::Black),
        '1' => parse_quote!(::mc_chat::TextColor::DarkBlue),
        '2' => parse_quote!(::mc_chat::TextColor::DarkGreen),
        '3' => parse_quote!(::mc_chat::TextColor::DarkCyan),
        '4' => parse_quote!(::mc_chat::TextColor::DarkRed),
        '5' => parse_quote!(::mc_chat::TextColor::Purple),
        '6' => parse_quote!(::mc_chat::TextColor::Gold),
        '7' => parse_quote!(::mc_chat::TextColor::Gray),
        '8' => parse_quote!(::mc_chat::TextColor::DarkGray),
        '9' => parse_quote!(::mc_chat::TextColor::Blue),
        'a' => parse_quote!(::mc_chat::TextColor::Green),
        'b' => parse_quote!(::mc_chat::TextColor::Cyan),
        'c' => parse_quote!(::mc_chat::TextColor::Red),
        'd' => parse_quote!(::mc_chat::TextColor::Pink),
        'e' => parse_quote!(::mc_chat::TextColor::Yellow),
        'f' => parse_quote!(::mc_chat::TextColor::White),
        _ => abort!(span, "Function contract broken, unrecognized color code"),
    })
}

#[derive(Default)]
pub struct ExpandedChatPart {
    pub tokens: Option<TokenStream>,
    pub color: Option<ExprPath>,
    pub extra_style: HashSet<char>,
    pub children: Vec<ExpandedChatPart>,
}

impl ExpandedChatPart {
    pub fn new(tokens: TokenStream) -> Self {
        Self {
            tokens: Some(tokens),
            ..Default::default()
        }
    }
    pub fn is_placeholder(&self) -> bool {
        self.color.is_none() && self.extra_style.is_empty() && self.tokens.is_none()
    }
}

impl Debug for ExpandedChatPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExpandedChatPart")
            .field("tokens", &self.tokens.as_ref().map(|t| t.to_string()))
            .field("color", &self.color.as_ref())
            .field("extra_style", &self.extra_style)
            .field("children", &self.children)
            .finish()
    }
}

impl ToTokens for ExpandedChatPart {
    fn to_tokens(&self, token_stream: &mut TokenStream) {
        let mut tokens: TokenStream = self.tokens.clone().unwrap_or(quote!(::mc_chat::Chat::text("")));
        if let Some(ref color) = self.color {
            tokens = quote!(#tokens.color(#color));
        }
        for code in &self.extra_style {
            match code {
                'k' => tokens = quote!(#tokens.obfuscated(true)),
                'l' => tokens = quote!(#tokens.bold(true)),
                'm' => tokens = quote!(#tokens.strikethrough(true)),
                'n' => tokens = quote!(#tokens.underlined(true)),
                'o' => tokens = quote!(#tokens.italic(true)),
                _ => panic!("Invalid non-color code!!"),
            }
        }
        for child in &self.children {
            tokens = quote!(#tokens.child(#child));
        }
        token_stream.extend(tokens);
    }
}

pub struct LegacyChat {
    pub pattern: LitStr,
    pub comma: Token![,],
    pub chat_parts: Punctuated::<ChatPart, Token![,]>,
}

impl Parse for LegacyChat {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let pattern = input.parse()?;
        let comma = input.parse()?;
        let chat_parts = Punctuated::parse_terminated(input)?;
        Ok(LegacyChat { pattern, comma, chat_parts, })
    }
}

pub enum ChatPart {
    Literal(LitStr),
    Variable(Expr),
}

impl Parse for ChatPart {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            Ok(ChatPart::Literal(input.parse()?))
        } else {
            Ok(ChatPart::Variable(input.parse()?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod chat {
        use super::*;

        #[test]
        fn plain_text() {
            let text: LegacyChat = parse_quote!("§", "Hello world!");
            assert_eq!("§", &text.pattern.value());
            assert_eq!(1, text.chat_parts.len());
        }

        #[test]
        fn complex_text() {
            let text: LegacyChat = parse_quote!("§&", "§&4Hello to §&b§&5world", variable, "§&r§&4!!");
            assert_eq!("§&", &text.pattern.value());
            assert_eq!(3, text.chat_parts.len());
        }
    }

    mod tree {
        use super::*;

        #[test]
        fn plain_text() {
            let text: LegacyChat = parse_quote!("§", "Hello world!");
            let tree = map_to_tree(text).unwrap();
            assert_eq!(0, tree.children.len());
            assert!(tree.color.is_none() && tree.extra_style.is_empty());
        }

        #[test]
        fn one_color() {
            let text: LegacyChat = parse_quote!("§", "§4Hello world!");
            let tree = map_to_tree(text).unwrap();
            assert_eq!(0, tree.children.len());
            assert!(tree.color.is_some());
        }

        #[test]
        fn two_colors() {
            let text: LegacyChat = parse_quote!("§", "§4Hello §5world!");
            let tree = map_to_tree(text).unwrap();
            assert_eq!(2, tree.children.len());
            assert!(tree.children[0].color.is_some() && tree.children[0].extra_style.is_empty());
            assert!(tree.children[1].color.is_some() && tree.children[1].extra_style.is_empty());
        }

        #[test]
        fn two_colors_middle() {
            let text: LegacyChat = parse_quote!("§", "Testing §4Hello §5world!");
            let tree = map_to_tree(text).unwrap();
            assert_eq!(3, tree.children.len());
            assert!(tree.children[0].color.is_none() && tree.children[0].extra_style.is_empty());
            assert!(tree.children[1].color.is_some() && tree.children[1].extra_style.is_empty());
            assert!(tree.children[2].color.is_some() && tree.children[2].extra_style.is_empty());
        }

        #[test]
        fn single_bold() {
            let text: LegacyChat = parse_quote!("§", "§lTesting");
            let tree = map_to_tree(text).unwrap();
            assert_eq!(0, tree.children.len());
            assert_eq!(1, tree.extra_style.len());
            assert!(tree.color.is_none());
        }

        #[test]
        fn bold_then_color() {
            let text: LegacyChat = parse_quote!("§", "§lTesting §4sequence");
            let tree = map_to_tree(text).unwrap();
            assert_eq!(1, tree.children.len());
            assert_eq!(1, tree.extra_style.len());
            assert!(tree.color.is_none());
            assert_eq!(0, tree.children[0].children.len());
            assert!(tree.children[0].extra_style.is_empty());
            assert!(tree.children[0].color.is_some());
        }

        #[test]
        fn color_bold_color() {
            let text: LegacyChat = parse_quote!("§", "§2Color §ltesting §4sequence");
            let tree = map_to_tree(text).unwrap();
            assert_eq!(1, tree.children.len());
            assert!(tree.extra_style.is_empty());
            assert!(tree.color.is_some());
            assert_eq!(1, tree.children[0].children.len());
            assert_eq!(1, tree.children[0].extra_style.len());
            assert!(tree.children[0].color.is_none());
            assert_eq!(0, tree.children[0].children[0].children.len());
            assert!(tree.children[0].children[0].extra_style.is_empty());
            assert!(tree.children[0].children[0].color.is_some());
        }

        #[test]
        fn mixed() {
            let text: LegacyChat = parse_quote!("§", "§2§3§4§l§kTesting §l§l§2overly §7much");
            let tree = map_to_tree(text).unwrap();
            assert_eq!(1, tree.children.len());
            assert_eq!(2, tree.children[0].children.len());
        }

        #[test]
        fn end() {
            let text: LegacyChat = parse_quote!("§", "Test end §6§l§6§k");
            let tree = map_to_tree(text).unwrap();
            assert_eq!(0, tree.children.len());
            assert!(tree.color.is_none());
            assert!(tree.extra_style.is_empty());
        }

        #[test]
        fn reset() {
            let text: LegacyChat = parse_quote!("§", "§lTest §r§2reset");
            let tree = map_to_tree(text).unwrap();
            assert_eq!(2, tree.children.len());
            assert!(tree.children[0].color.is_none());
            assert!(tree.children[1].extra_style.is_empty());
        }
    }
}
