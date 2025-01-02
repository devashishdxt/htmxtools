use std::{iter::once, ops::Deref};

#[cfg(feature = "axum")]
use axum_core::response::{IntoResponse, IntoResponseParts, Response, ResponseParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header, HeaderName, HeaderValue};

use crate::util::{iter::IterExt, value_string::HeaderValueString};

static HX_RETARGET: HeaderName = HeaderName::from_static("hx-retarget");

/// A CSS selector that updates the target of the content update to a different element on the page.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HxRetarget(HeaderValueString);

impl HxRetarget {
    /// Create a new `HxRetarget` from a static string.
    ///
    /// # Panic
    ///
    /// Panics if the static string is not a legal header value.
    pub const fn from_static(src: &'static str) -> Self {
        Self(HeaderValueString::from_static(src))
    }

    /// Create a new `HxRetarget` from a string.
    pub fn from_string(src: String) -> Option<Self> {
        HeaderValueString::from_string(src).map(Self)
    }

    /// View this `HxRetarget` as a `&str`.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Deref for HxRetarget {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

#[cfg(feature = "axum")]
impl IntoResponseParts for HxRetarget {
    type Error = <TypedHeader<Self> as IntoResponseParts>::Error;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        TypedHeader(self).into_response_parts(res)
    }
}

#[cfg(feature = "axum")]
impl IntoResponse for HxRetarget {
    fn into_response(self) -> Response {
        TypedHeader(self).into_response()
    }
}

impl Header for HxRetarget {
    fn name() -> &'static HeaderName {
        &HX_RETARGET
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
