//! Internal abstraction for string cloning efficiency.
//!
//! In Minecraft's chat data type, strings get nested a lot. Chat gets
//! used a lot, so this needs to be optimized. There are two main options for
//! efficient, immutable strings: [`Arc<str>`](std::sync::Arc) and [`Box<str>`].
//!
//! In a typical server's perspective, there are the following scenarios:
//! - A player sends a message to the server -> the server broadcasts this message.
//! - The server sends a message to all players (usually more [`Style`](crate::Style) applied).
//!
//! For an optimized server, this will be done asynchronously. While [`Arc<str>`](std::sync::Arc)
//! seems interesting for lots of cloning without overhead, it's less efficient
//! to create a lot of small reference counted objects instead of wrapping the
//! whole chat component in a single [`Arc`](std::sync::Arc). This means that [`FrozenStr`] is
//! implemented as a simple wrapper around [`Box<str>`].
//!

use std::{fmt::Display, ops::Deref};

use serde::{de::Visitor, Deserialize, Serialize};

/// Efficient immutable string.
///
/// See the [module](self)'s documentation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FrozenStr {
    str: Box<str>,
}

impl Display for FrozenStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.str.fmt(f)
    }
}

impl<T> From<T> for FrozenStr
where
    T: Into<Box<str>>,
{
    fn from(str: T) -> Self {
        Self { str: str.into() }
    }
}

impl Deref for FrozenStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.str.deref()
    }
}

#[cfg(feature = "serde")]
impl Serialize for FrozenStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.deref().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for FrozenStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StrVisitor;

        impl<'de> Visitor<'de> for StrVisitor {
            type Value = FrozenStr;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("A string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v.into())
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v.into())
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v.into())
            }
        }

        deserializer.deserialize_string(StrVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};

    use super::*;

    #[test]
    fn test_serde() {
        let str: FrozenStr = "Hello world".into();
        assert_tokens(&str, &[Token::BorrowedStr("Hello world")]);
    }
}
