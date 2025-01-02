use std::{iter::once, ops::Deref};

#[cfg(feature = "axum")]
use axum_core::extract::{FromRequestParts, OptionalFromRequestParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header, HeaderName, HeaderValue};
#[cfg(feature = "axum")]
use http::request::Parts;

use crate::util::{iter::IterExt, value_string::HeaderValueString};

static HX_PROMPT: HeaderName = HeaderName::from_static("hx-prompt");

/// The user response to an [`hx-prompt`](https://htmx.org/attributes/hx-prompt/).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HxPrompt(HeaderValueString);

impl HxPrompt {
    /// Create a new `HxPrompt` from a static string.
    ///
    /// # Panic
    ///
    /// Panics if the static string is not a legal header value.
    pub const fn from_static(src: &'static str) -> Self {
        Self(HeaderValueString::from_static(src))
    }

    /// View this `HxPrompt` as a `&str`.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Deref for HxPrompt {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

#[cfg(feature = "axum")]
impl<S> FromRequestParts<S> for HxPrompt
where
    S: Send + Sync,
{
    type Rejection = <TypedHeader<Self> as FromRequestParts<S>>::Rejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        <TypedHeader<Self> as FromRequestParts<S>>::from_request_parts(parts, state)
            .await
            .map(|header| header.0)
    }
}

#[cfg(feature = "axum")]
impl<S> OptionalFromRequestParts<S> for HxPrompt
where
    S: Send + Sync,
{
    type Rejection = <TypedHeader<Self> as OptionalFromRequestParts<S>>::Rejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        <TypedHeader<Self> as OptionalFromRequestParts<S>>::from_request_parts(parts, state)
            .await
            .map(|optional_header| optional_header.map(|header| header.0))
    }
}

impl Header for HxPrompt {
    fn name() -> &'static HeaderName {
        &HX_PROMPT
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .just_one()
            .map(|value| HeaderValueString::try_from_header_value(value).map(Self))
            .transpose()?
            .ok_or_else(Error::invalid)
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        values.extend(once(self.0.as_header_value().clone()));
    }
}
