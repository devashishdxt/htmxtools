#[cfg(feature = "axum")]
use axum_core::extract::{FromRequestParts, OptionalFromRequestParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header, HeaderName, HeaderValue};
#[cfg(feature = "axum")]
use http::request::Parts;

#[cfg(feature = "auto-vary")]
use crate::auto_vary::{AutoVaryAdd, HxRequestHeader};
use crate::util::iter::IterExt;

static HX_REQUEST_TYPE: HeaderName = HeaderName::from_static("hx-request-type");

/// Indicates whether htmx is requesting a partial page update or full page content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HxRequestType {
    /// The request targets a specific element on the page (most common case)
    Partial,

    /// The request targets the entire body element (including via
    /// [`hx-boost`](https://four.htmx.org/attributes/hx-boost/)) or uses `hx-select` to extract content
    Full,
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl<S> FromRequestParts<S> for HxRequestType
where
    S: Send + Sync,
{
    type Rejection = <TypedHeader<Self> as FromRequestParts<S>>::Rejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        #[cfg(feature = "auto-vary")]
        parts.auto_vary_add(HxRequestHeader::RequestType);

        <TypedHeader<Self> as FromRequestParts<S>>::from_request_parts(parts, state)
            .await
            .map(|header| header.0)
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl<S> OptionalFromRequestParts<S> for HxRequestType
where
    S: Send + Sync,
{
    type Rejection = <TypedHeader<Self> as OptionalFromRequestParts<S>>::Rejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        #[cfg(feature = "auto-vary")]
        parts.auto_vary_add(HxRequestHeader::RequestType);

        <TypedHeader<Self> as OptionalFromRequestParts<S>>::from_request_parts(parts, state)
            .await
            .map(|optional_header| optional_header.map(|header| header.0))
    }
}

impl Header for HxRequestType {
    fn name() -> &'static HeaderName {
        &HX_REQUEST_TYPE
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .just_one()
            .and_then(|value| {
                if value == "partial" {
                    Some(Self::Partial)
                } else if value == "full" {
                    Some(Self::Full)
                } else {
                    None
                }
            })
            .ok_or_else(Error::invalid)
    }

    fn encode<E: Extend<HeaderValue>>(&self, _: &mut E) {
        // This is a request header, so encoding it is not valid.
        // Do nothing
    }
}
