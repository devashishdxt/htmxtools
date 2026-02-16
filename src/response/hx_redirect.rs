use std::iter::once;

#[cfg(feature = "axum")]
use axum_core::response::{IntoResponse, IntoResponseParts, Response, ResponseParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header, HeaderName};
use http::{HeaderValue, Uri};

use crate::util::uri::UriExt;

static HX_REDIRECT: HeaderName = HeaderName::from_static("hx-redirect");

/// Can be used to do a client-side redirect to a new location.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HxRedirect(pub Uri);

impl HxRedirect {
    /// Creates a new `HxRedirect` instance from a URI.
    pub fn new(uri: Uri) -> Self {
        Self(uri)
    }
}

impl From<Uri> for HxRedirect {
    fn from(uri: Uri) -> Self {
        Self(uri)
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponseParts for HxRedirect {
    type Error = <TypedHeader<Self> as IntoResponseParts>::Error;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        TypedHeader(self).into_response_parts(res)
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponse for HxRedirect {
    fn into_response(self) -> Response {
        TypedHeader(self).into_response()
    }
}

impl Header for HxRedirect {
    fn name() -> &'static HeaderName {
        &HX_REDIRECT
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
        if let Some(value) = HeaderValue::from_uri(&self.0) {
            values.extend(once(value));
        }
    }
}
