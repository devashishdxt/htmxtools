use std::iter::once;

#[cfg(feature = "axum")]
use axum_core::response::{IntoResponse, IntoResponseParts, Response, ResponseParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header, HeaderName, HeaderValue};

use crate::util::value_string::HeaderValueString;

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

    /// Create a new `HxRetarget` from a `&str`.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(src: &str) -> Option<Self> {
        HeaderValueString::from_str(src).map(Self)
    }

    /// Create a new `HxRetarget` from a `String`.
    pub fn from_string(src: String) -> Option<Self> {
        HeaderValueString::from_string(src).map(Self)
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponseParts for HxRetarget {
    type Error = <TypedHeader<Self> as IntoResponseParts>::Error;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        TypedHeader(self).into_response_parts(res)
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponse for HxRetarget {
    fn into_response(self) -> Response {
        TypedHeader(self).into_response()
    }
}

impl Header for HxRetarget {
    fn name() -> &'static HeaderName {
        &HX_RETARGET
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
