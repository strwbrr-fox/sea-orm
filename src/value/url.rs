use core::{
    borrow::{Borrow, BorrowMut},
    fmt,
    ops::{Deref, DerefMut, Index},
    str::FromStr,
};

pub use ::url::Position as UrlPosition;

use crate::{
    ActiveValue, ColIdx, DbErr, IntoActiveValue, QueryResult, TryGetError, TryGetable, Value,
    sea_query::{ArrayType, ColumnType, Nullable, ValueType, ValueTypeErr},
};

/// A [`Url`](::url::Url) mapped to a [`String`] in a database
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Url(::url::Url);

impl From<::url::ParseError> for DbErr {
    fn from(value: ::url::ParseError) -> Self {
        DbErr::TryIntoErr {
            from: "String",
            into: stringify!(Url),
            source: std::sync::Arc::from(value),
        }
    }
}

impl Deref for Url {
    type Target = ::url::Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Url {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<I> Index<I> for Url
where
    ::url::Url: Index<I>,
{
    type Output = <::url::Url as Index<I>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.0.index(index)
    }
}

impl AsRef<::url::Url> for Url {
    fn as_ref(&self) -> &::url::Url {
        &self.0
    }
}

impl AsMut<::url::Url> for Url {
    fn as_mut(&mut self) -> &mut ::url::Url {
        &mut self.0
    }
}

impl Borrow<::url::Url> for Url {
    fn borrow(&self) -> &::url::Url {
        &self.0
    }
}

impl BorrowMut<::url::Url> for Url {
    fn borrow_mut(&mut self) -> &mut ::url::Url {
        &mut self.0
    }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<::url::Url> for Url {
    fn from(value: ::url::Url) -> Self {
        Self(value)
    }
}

impl From<Url> for ::url::Url {
    fn from(value: Url) -> Self {
        value.0
    }
}

impl From<Url> for String {
    fn from(value: Url) -> Self {
        String::from(value.0)
    }
}

impl fmt::Debug for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl TryGetable for Url {
    fn try_get_by<I: ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
        let string = <String as TryGetable>::try_get_by(res, idx)?;
        Self::from_str(&string).map_err(|err| {
            TryGetError::DbErr(DbErr::TryIntoErr {
                from: "String",
                into: stringify!(Url),
                source: std::sync::Arc::new(err),
            })
        })
    }
}

impl FromStr for Url {
    type Err = ::url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::url::Url::from_str(s).map(Self)
    }
}

impl ValueType for Url {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        let string = <String as ValueType>::try_from(v)?;
        Self::from_str(&string).map_err(|_| ValueTypeErr)
    }

    fn type_name() -> String {
        stringify!(Url).to_owned()
    }

    fn array_type() -> ArrayType {
        <String as ValueType>::array_type()
    }

    fn column_type() -> ColumnType {
        <String as ValueType>::column_type()
    }
}

impl Nullable for Url {
    fn null() -> Value {
        <String as Nullable>::null()
    }
}

impl IntoActiveValue<Url> for Url {
    fn into_active_value(self) -> ActiveValue<Url> {
        ActiveValue::Set(self)
    }
}

impl From<Url> for Value {
    fn from(value: Url) -> Self {
        Value::String(Some(String::from(value.0)))
    }
}

#[cfg(feature = "with-json")]
impl serde::Serialize for Url {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "with-json")]
impl<'de> serde::Deserialize<'de> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        ::url::Url::deserialize(deserializer).map(Self)
    }
}
