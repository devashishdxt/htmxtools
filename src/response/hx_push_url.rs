use std::iter::once;

#[cfg(feature = "axum")]
use axum_core::response::{IntoResponse, IntoResponseParts, Response, ResponseParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header};
use http::{HeaderName, HeaderValue, Uri};

use crate::util::{iter::IterExt, uri::UriExt};

const FALSE: HeaderValue = HeaderValue::from_static("false");

static HX_PUSH_URL: HeaderName = HeaderName::from_static("hx-push-url");

/// Pushes a new url into the history stack.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HxPushUrl(HxPushUrlValue);

impl HxPushUrl {
    /// Prevents the browser’s history from being updated.
    pub const fn prevent() -> Self {
        Self(HxPushUrlValue::Prevent)
    }

    /// A URL to be pushed into the location bar. This may be relative or absolute, as per
    /// [`history.pushState()`](https://developer.mozilla.org/en-US/docs/Web/API/History/pushState).
    pub fn url(uri: Uri) -> Self {
        Self(HxPushUrlValue::Url(uri))
    }

    /// Get the value of the `HxPushUrl`. If `None`, the browser’s history will not be updated.
    pub fn get(&self) -> Option<&Uri> {
        match &self.0 {
            HxPushUrlValue::Prevent => None,
            HxPushUrlValue::Url(uri) => Some(uri),
        }
    }

    fn from_header_value(value: &HeaderValue) -> Result<Self, Error> {
        if value == FALSE {
            Ok(Self(HxPushUrlValue::Prevent))
        } else {
            let uri = value.to_uri()?;
            Ok(Self(HxPushUrlValue::Url(uri)))
        }
    }

    fn to_header_value(&self) -> Option<HeaderValue> {
        match &self.0 {
            HxPushUrlValue::Prevent => Some(FALSE.clone()),
            HxPushUrlValue::Url(uri) => HeaderValue::from_uri(uri),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum HxPushUrlValue {
    Prevent,
    Url(Uri),
}

#[cfg(feature = "axum")]
impl IntoResponseParts for HxPushUrl {
    type Error = <TypedHeader<Self> as IntoResponseParts>::Error;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        TypedHeader(self).into_response_parts(res)
    }
}

#[cfg(feature = "axum")]
impl IntoResponse for HxPushUrl {
    fn into_response(self) -> Response {
        TypedHeader(self).into_response()
    }
}

impl Header for HxPushUrl {
    fn name() -> &'static HeaderName {
        &HX_PUSH_URL
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values.just_one().ok_or_else(Error::invalid)?;
        Self::from_header_value(value)
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        values.extend(once(
            self.to_header_value()
                .expect("invalid value for HX-Push-URL"),
        ));
    }
}
