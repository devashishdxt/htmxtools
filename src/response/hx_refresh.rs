use std::iter::once;

#[cfg(feature = "axum")]
use axum_core::response::{IntoResponse, IntoResponseParts, Response, ResponseParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header, HeaderName, HeaderValue};

static HX_REFRESH: HeaderName = HeaderName::from_static("hx-refresh");

/// If set the client-side will do a full refresh of the page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HxRefresh;

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponseParts for HxRefresh {
    type Error = <TypedHeader<Self> as IntoResponseParts>::Error;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        TypedHeader(self).into_response_parts(res)
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponse for HxRefresh {
    fn into_response(self) -> Response {
        TypedHeader(self).into_response()
    }
}

impl Header for HxRefresh {
    fn name() -> &'static HeaderName {
        &HX_REFRESH
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
        values.extend(once(HeaderValue::from_static("true")));
    }
}
