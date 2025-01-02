use std::{
    iter::once,
    ops::{Deref, DerefMut},
};

#[cfg(feature = "axum")]
use axum_core::response::{IntoResponse, IntoResponseParts, Response, ResponseParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header, HeaderName};
use http::{HeaderValue, Uri};

use crate::util::{iter::IterExt, uri::UriExt};

static HX_REDIRECT: HeaderName = HeaderName::from_static("hx-redirect");

/// Can be used to do a client-side redirect to a new location.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HxRedirect(Uri);

impl Deref for HxRedirect {
    type Target = Uri;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HxRedirect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Uri> for HxRedirect {
    fn from(uri: Uri) -> Self {
        Self(uri)
    }
}

impl From<HxRedirect> for Uri {
    fn from(hx_redirect: HxRedirect) -> Self {
        hx_redirect.0
    }
}

#[cfg(feature = "axum")]
impl IntoResponseParts for HxRedirect {
    type Error = <TypedHeader<Self> as IntoResponseParts>::Error;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        TypedHeader(self).into_response_parts(res)
    }
}

#[cfg(feature = "axum")]
impl IntoResponse for HxRedirect {
    fn into_response(self) -> Response {
        TypedHeader(self).into_response()
    }
}

impl Header for HxRedirect {
    fn name() -> &'static HeaderName {
        &HX_REDIRECT
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

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        let value = HeaderValue::from_uri(&self.0).expect("invalid value for HX-Redirect");
        values.extend(once(value));
    }
}
