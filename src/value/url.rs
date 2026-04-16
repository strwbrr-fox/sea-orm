use core::{
    fmt,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use ::url::Url;

use crate::{
    ActiveValue, ColIdx, DbErr, IntoActiveValue, QueryResult, TryGetError, TryGetable, Value,
    sea_query::{ArrayType, ColumnType, Nullable, ValueType, ValueTypeErr},
};

/// A [`Url`] mapped to a [`String`] in a database
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TextUrl(pub Url);

impl From<::url::ParseError> for DbErr {
    fn from(value: ::url::ParseError) -> Self {
        DbErr::TryIntoErr {
            from: "String",
            into: stringify!(TextUrl),
            source: std::sync::Arc::from(value),
        }
    }
}

impl Deref for TextUrl {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TextUrl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<Url> for TextUrl {
    fn as_ref(&self) -> &Url {
        &self.0
    }
}

impl AsMut<Url> for TextUrl {
    fn as_mut(&mut self) -> &mut Url {
        &mut self.0
    }
}

impl AsRef<str> for TextUrl {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<Url> for TextUrl {
    fn from(value: Url) -> Self {
        Self(value)
    }
}

impl From<TextUrl> for Url {
    fn from(value: TextUrl) -> Self {
        value.0
    }
}

impl From<TextUrl> for String {
    fn from(value: TextUrl) -> Self {
        String::from(value.0)
    }
}

impl fmt::Debug for TextUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for TextUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl TryGetable for TextUrl {
    fn try_get_by<I: ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
        let string = <String as TryGetable>::try_get_by(res, idx)?;
        Self::from_str(&string).map_err(|err| {
            TryGetError::DbErr(DbErr::TryIntoErr {
                from: "String",
                into: stringify!(TextUrl),
                source: std::sync::Arc::new(err),
            })
        })
    }
}

impl FromStr for TextUrl {
    type Err = ::url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Url::from_str(s).map(Self)
    }
}

impl ValueType for TextUrl {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        let string = <String as ValueType>::try_from(v)?;
        Self::from_str(&string).map_err(|_| ValueTypeErr)
    }

    fn type_name() -> String {
        stringify!(TextUrl).to_owned()
    }

    fn array_type() -> ArrayType {
        <String as ValueType>::array_type()
    }

    fn column_type() -> ColumnType {
        <String as ValueType>::column_type()
    }
}

impl Nullable for TextUrl {
    fn null() -> Value {
        <String as Nullable>::null()
    }
}

impl IntoActiveValue<TextUrl> for TextUrl {
    fn into_active_value(self) -> ActiveValue<TextUrl> {
        ActiveValue::Set(self)
    }
}

impl From<TextUrl> for Value {
    fn from(value: TextUrl) -> Self {
        Value::String(Some(String::from(value.0)))
    }
}

#[cfg(feature = "with-json")]
impl serde::Serialize for TextUrl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "with-json")]
impl<'de> serde::Deserialize<'de> for TextUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Url::deserialize(deserializer).map(Self)
    }
}
