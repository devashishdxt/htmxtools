use std::ops::{Deref, DerefMut};

#[cfg(feature = "axum")]
use axum_core::extract::{FromRequestParts, OptionalFromRequestParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header, HeaderName};
#[cfg(feature = "axum")]
use http::request::Parts;
use http::{HeaderValue, Uri};

#[cfg(feature = "auto-vary")]
use crate::auto_vary::{AutoVaryAdd, HxRequestHeader};
use crate::util::{iter::IterExt, uri::UriExt};

static HX_CURRENT_URL: HeaderName = HeaderName::from_static("hx-current-url");

/// The current URL of the browser.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HxCurrentUrl(Uri);

impl HxCurrentUrl {
    /// Returns the current URL of the browser.
    pub fn as_uri(&self) -> &Uri {
        &self.0
    }
}

impl Deref for HxCurrentUrl {
    type Target = Uri;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HxCurrentUrl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Uri> for HxCurrentUrl {
    fn from(uri: Uri) -> Self {
        Self(uri)
    }
}

impl From<HxCurrentUrl> for Uri {
    fn from(hx_current_url: HxCurrentUrl) -> Self {
        hx_current_url.0
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl<S> FromRequestParts<S> for HxCurrentUrl
where
    S: Send + Sync,
{
    type Rejection = <TypedHeader<Self> as FromRequestParts<S>>::Rejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        #[cfg(feature = "auto-vary")]
        parts.auto_vary_add(HxRequestHeader::CurrentUrl);

        <TypedHeader<Self> as FromRequestParts<S>>::from_request_parts(parts, state)
            .await
            .map(|header| header.0)
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl<S> OptionalFromRequestParts<S> for HxCurrentUrl
where
    S: Send + Sync,
{
    type Rejection = <TypedHeader<Self> as OptionalFromRequestParts<S>>::Rejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        #[cfg(feature = "auto-vary")]
        parts.auto_vary_add(HxRequestHeader::CurrentUrl);

        <TypedHeader<Self> as OptionalFromRequestParts<S>>::from_request_parts(parts, state)
            .await
            .map(|optional_header| optional_header.map(|header| header.0))
    }
}

impl Header for HxCurrentUrl {
    fn name() -> &'static HeaderName {
        &HX_CURRENT_URL
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .just_one()
            .ok_or_else(Error::invalid)?
            .to_uri()
            .map(Self)
    }

    fn encode<E: Extend<HeaderValue>>(&self, _: &mut E) {
        // This is a request header, so encoding it is not valid.
        // Do nothing
    }
}
