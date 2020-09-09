use serde::ser::{Serialize, Serializer};
use std::convert::TryFrom;

use crate::Error;
use crate::{Signature, Type, Value};

/// A helper type to wrap Option<T> (GVariant's Maybe type) in [`Value`].
///
/// API is provided to convert from, and to Option<T>.
///
/// [`Value`]: enum.Value.html
#[derive(Debug, Clone, PartialEq)]
pub struct Maybe<'a> {
    value: Box<Option<Value<'a>>>,
    value_signature: Signature<'a>,
    signature: Signature<'static>,
}

impl<'a> Maybe<'a> {
    /// Get a reference to underlying value.
    pub fn inner(&self) -> &Option<Value<'a>> {
        &self.value
    }

    /// Create a new Just (Some) `Maybe`.
    pub fn just(value: Value<'a>) -> Self {
        let value_signature = value.value_signature().to_owned();
        let signature = create_signature(&value_signature);
        Self {
            value_signature,
            signature,
            value: Box::new(Some(value)),
        }
    }

    /// Create a new Nothing (None) `Maybe`, given the signature of the type.
    pub fn nothing<'s: 'a>(value_signature: Signature<'s>) -> Self {
        let signature = create_signature(&value_signature);
        Self {
            value_signature,
            signature,
            value: Box::new(None),
        }
    }

    /// Get the inner value as a concrete type
    pub fn get<T>(self) -> core::result::Result<Option<T>, Error>
    where
        T: TryFrom<Value<'a>>,
    {
        self.value
            .map(|v| v.downcast().ok_or(Error::IncorrectType))
            .transpose()
    }

    /// Get the signature of `Maybe`.
    pub fn signature(&self) -> Signature<'static> {
        self.signature.clone()
    }

    /// Get the signature of the potential value in the `Maybe`.
    pub fn value_signature(&self) -> &Signature {
        &self.value_signature
    }

    pub(crate) fn to_owned(&self) -> Maybe<'static> {
        Maybe {
            value_signature: self.value_signature.to_owned(),
            value: Box::new(self.value.clone().map(|v| v.to_owned())),
            signature: self.signature.to_owned(),
        }
    }
}

impl<'a, T> From<Option<T>> for Maybe<'a>
where
    T: Type + Into<Value<'a>>,
{
    fn from(value: Option<T>) -> Self {
        value
            .map(|v| Self::just(Value::new(v)))
            .unwrap_or_else(|| Self::nothing(T::signature()))
    }
}

impl<'a, T> From<&Option<T>> for Maybe<'a>
where
    T: Type + Into<Value<'a>> + Clone,
{
    fn from(value: &Option<T>) -> Self {
        value
            .as_ref()
            .map(|v| Self::just(Value::new(v.clone())))
            .unwrap_or_else(|| Self::nothing(T::signature()))
    }
}

// This would be great but somehow it conflicts with some blanket generic implementations from
// core:
//
// impl<'a, T> TryFrom<Maybe<'a>> for Option<T>

impl<'a> Serialize for Maybe<'a> {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &*self.value {
            Some(value) => value.serialize_value_as_some(serializer),
            None => serializer.serialize_none(),
        }
    }
}

fn create_signature(value_signature: &Signature) -> Signature<'static> {
    Signature::from_string_unchecked(format!("m{}", value_signature))
}
