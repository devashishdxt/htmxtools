use std::iter::once;

#[cfg(feature = "axum")]
use axum_core::response::{IntoResponse, IntoResponseParts, Response, ResponseParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header};
use http::{HeaderName, HeaderValue, Uri};

use crate::util::uri::UriExt;

const FALSE: HeaderValue = HeaderValue::from_static("false");
const TRUE: HeaderValue = HeaderValue::from_static("true");

static HX_PUSH_URL: HeaderName = HeaderName::from_static("hx-push-url");

/// Pushes a new url into the history stack.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HxPushUrl {
    /// Pushes the current request's url into the history stack.
    True,

    /// Disables pushing a new url into the history stack.
    False,

    /// Pushes a new url into the history stack.
    Uri(Uri),
}

impl HxPushUrl {
    fn to_header_value(&self) -> Option<HeaderValue> {
        match self {
            Self::False => Some(FALSE),
            Self::True => Some(TRUE),
            Self::Uri(uri) => HeaderValue::from_uri(uri),
        }
    }
}

impl From<bool> for HxPushUrl {
    fn from(value: bool) -> Self {
        if value { Self::True } else { Self::False }
    }
}

impl From<Uri> for HxPushUrl {
    fn from(uri: Uri) -> Self {
        Self::Uri(uri)
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponseParts for HxPushUrl {
    type Error = <TypedHeader<Self> as IntoResponseParts>::Error;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        TypedHeader(self).into_response_parts(res)
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponse for HxPushUrl {
    fn into_response(self) -> Response {
        TypedHeader(self).into_response()
    }
}

impl Header for HxPushUrl {
    fn name() -> &'static HeaderName {
        &HX_PUSH_URL
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
        if let Some(header_value) = self.to_header_value() {
            values.extend(once(header_value));
        }
    }
}
