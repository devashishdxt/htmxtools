use std::{iter::once, ops::Deref};

#[cfg(feature = "axum")]
use axum_core::response::{IntoResponse, IntoResponseParts, Response, ResponseParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header, HeaderName, HeaderValue};

use crate::util::{iter::IterExt, value_string::HeaderValueString};

static HX_RESELECT: HeaderName = HeaderName::from_static("hx-reselect");

/// A CSS selector that allows you to choose which part of the response is used to be swapped in. Overrides an existing
/// [`hx-select`](https://htmx.org/attributes/hx-select/) on the triggering element.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HxReselect(HeaderValueString);

impl HxReselect {
    /// Create a new `HxReselect` from a static string.
    ///
    /// # Panic
    ///
    /// Panics if the static string is not a legal header value.
    pub const fn from_static(src: &'static str) -> Self {
        Self(HeaderValueString::from_static(src))
    }

    /// Create a new `HxReselect` from a string.
    pub fn from_string(src: String) -> Option<Self> {
        HeaderValueString::from_string(src).map(Self)
    }

    /// View this `HxReselect` as a `&str`.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Deref for HxReselect {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

#[cfg(feature = "axum")]
impl IntoResponseParts for HxReselect {
    type Error = <TypedHeader<Self> as IntoResponseParts>::Error;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        TypedHeader(self).into_response_parts(res)
    }
}

#[cfg(feature = "axum")]
impl IntoResponse for HxReselect {
    fn into_response(self) -> Response {
        TypedHeader(self).into_response()
    }
}

impl Header for HxReselect {
    fn name() -> &'static HeaderName {
        &HX_RESELECT
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
