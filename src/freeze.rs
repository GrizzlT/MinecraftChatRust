use std::{sync::Arc, ops::Deref, fmt::Display};

use serde::{Serialize, Deserialize};

/// See the [module](self)'s documentation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FrozenStr {
    str: Arc<str>,
}

impl Display for FrozenStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.str.fmt(f)
    }
}

impl<T> From<T> for FrozenStr
where
    T: Into<Arc<str>>,
{
    fn from(str: T) -> Self {
        Self {
            str: str.into()
        }
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
        S: serde::Serializer
    {
        self.deref().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for FrozenStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let str = <&str>::deserialize(deserializer)?;
        Ok(str.into())
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};

    use super::*;

    #[test]
    fn test_serde() {
        let str: FrozenStr = "Hello world".into();
        assert_tokens(&str, &[
            Token::BorrowedStr("Hello world"),
        ]);
    }
}
