use std::{borrow::Cow, sync::Arc, ops::Deref, fmt::Display};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub enum FreezeStr {
    Cow(Cow<'static, str>),
    Frozen(Arc<str>),
}

impl FreezeStr {
    pub fn freeze(&mut self) {
        match self {
            FreezeStr::Cow(str) => {
                let str = std::mem::take(str);
                *self = FreezeStr::Frozen(str.into());
            },
            FreezeStr::Frozen(_) => {},
        }
    }
}

impl Display for FreezeStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FreezeStr::Cow(str) => str.fmt(f),
            FreezeStr::Frozen(str) => str.fmt(f),
        }
    }
}

impl<T> From<T> for FreezeStr
where
    T: Into<Cow<'static, str>>,
{
    fn from(str: T) -> Self {
        Self::Cow(str.into())
    }
}

impl Deref for FreezeStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Cow(str) => str.deref(),
            Self::Frozen(str) => str.deref(),
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for FreezeStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        self.deref().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for FreezeStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let str = Cow::<'static, str>::deserialize(deserializer)?;
        Ok(Self::Cow(str))
    }
}
