#[cfg(feature = "axum")]
use axum_core::extract::{FromRequestParts, OptionalFromRequestParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header, HeaderName, HeaderValue};
#[cfg(feature = "axum")]
use http::request::Parts;

#[cfg(feature = "auto-vary")]
use crate::auto_vary::{AutoVaryAdd, HxRequestHeader};
use crate::util::{iter::IterExt, value_string::HeaderValueString};

static HX_TARGET: HeaderName = HeaderName::from_static("hx-target");

/// The identifier of the target element in format `tag#id` (e.g., `div#results`). `id` is optional.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HxTarget(HeaderValueString);

impl HxTarget {
    /// Returns the tag name of the target element.
    pub fn tag(&self) -> &str {
        if self.0.as_str().contains('#') {
            self.0.as_str().split_once('#').unwrap().0
        } else {
            self.0.as_str()
        }
    }

    /// Returns the id of the target element if it exists.
    pub fn id(&self) -> Option<&str> {
        if self.0.as_str().contains('#') {
            Some(self.0.as_str().split_once('#').unwrap().1)
        } else {
            None
        }
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl<S> FromRequestParts<S> for HxTarget
where
    S: Send + Sync,
{
    type Rejection = <TypedHeader<Self> as FromRequestParts<S>>::Rejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        #[cfg(feature = "auto-vary")]
        parts.auto_vary_add(HxRequestHeader::Target);

        <TypedHeader<Self> as FromRequestParts<S>>::from_request_parts(parts, state)
            .await
            .map(|header| header.0)
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl<S> OptionalFromRequestParts<S> for HxTarget
where
    S: Send + Sync,
{
    type Rejection = <TypedHeader<Self> as OptionalFromRequestParts<S>>::Rejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        #[cfg(feature = "auto-vary")]
        parts.auto_vary_add(HxRequestHeader::Target);

        <TypedHeader<Self> as OptionalFromRequestParts<S>>::from_request_parts(parts, state)
            .await
            .map(|optional_header| optional_header.map(|header| header.0))
    }
}

impl Header for HxTarget {
    fn name() -> &'static HeaderName {
        &HX_TARGET
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

    fn encode<E: Extend<HeaderValue>>(&self, _: &mut E) {
        // This is a request header, so encoding it is not valid.
        // Do nothing
    }
}
