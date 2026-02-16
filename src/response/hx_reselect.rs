use std::iter::once;

#[cfg(feature = "axum")]
use axum_core::response::{IntoResponse, IntoResponseParts, Response, ResponseParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header, HeaderName, HeaderValue};

use crate::util::value_string::HeaderValueString;

static HX_RESELECT: HeaderName = HeaderName::from_static("hx-reselect");

/// A CSS selector that allows you to choose which part of the response is used to be swapped in. Overrides an
/// existing [`hx-select`](https://four.htmx.org/attributes/hx-select/) on the triggering element.
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

    /// Create a new `HxReselect` from a `&str`.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(src: &str) -> Option<Self> {
        HeaderValueString::from_str(src).map(Self)
    }

    /// Create a new `HxReselect` from a `String`.
    pub fn from_string(src: String) -> Option<Self> {
        HeaderValueString::from_string(src).map(Self)
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponseParts for HxReselect {
    type Error = <TypedHeader<Self> as IntoResponseParts>::Error;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        TypedHeader(self).into_response_parts(res)
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponse for HxReselect {
    fn into_response(self) -> Response {
        TypedHeader(self).into_response()
    }
}

impl Header for HxReselect {
    fn name() -> &'static HeaderName {
        &HX_RESELECT
    }

    fn decode<'i, I>(_: &mut I) -> Result<Self, Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        // This is a response header, so decoding it is not valid.
        Err(Error::invalid())
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        values.extend(once(self.0.as_header_value().clone()));
    }
}
