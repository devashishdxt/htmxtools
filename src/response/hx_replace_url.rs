use std::iter::once;

#[cfg(feature = "axum")]
use axum_core::response::{IntoResponse, IntoResponseParts, Response, ResponseParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header};
use http::{HeaderName, HeaderValue, Uri};

use crate::util::{iter::IterExt, uri::UriExt};

const FALSE: HeaderValue = HeaderValue::from_static("false");

static HX_REPLACE_URL: HeaderName = HeaderName::from_static("hx-replace-url");

/// Replaces the current URL in the location bar.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HxReplaceUrl(HxReplaceUrlValue);

impl HxReplaceUrl {
    /// Prevents the browser’s current URL from being updated.
    pub fn prevent() -> Self {
        Self(HxReplaceUrlValue::Prevent)
    }

    /// A URL to replace the current URL in the location bar. This may be relative or absolute, as per
    /// [`history.replaceState()`](https://developer.mozilla.org/en-US/docs/Web/API/History/replaceState), but must have
    /// the same origin as the current URL.
    pub fn url(uri: Uri) -> Self {
        Self(HxReplaceUrlValue::Url(uri))
    }

    /// Get the value of the `HxReplaceUrl`. If `None`, the browser’s current URL will not be updated.
    pub fn get(&self) -> Option<&Uri> {
        match &self.0 {
            HxReplaceUrlValue::Prevent => None,
            HxReplaceUrlValue::Url(uri) => Some(uri),
        }
    }

    fn from_header_value(value: &HeaderValue) -> Result<Self, Error> {
        if value == FALSE {
            Ok(Self(HxReplaceUrlValue::Prevent))
        } else {
            let uri = value.to_uri()?;
            Ok(Self(HxReplaceUrlValue::Url(uri)))
        }
    }

    fn to_header_value(&self) -> Option<HeaderValue> {
        match &self.0 {
            HxReplaceUrlValue::Prevent => Some(FALSE.clone()),
            HxReplaceUrlValue::Url(uri) => HeaderValue::from_uri(uri),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum HxReplaceUrlValue {
    Prevent,
    Url(Uri),
}

#[cfg(feature = "axum")]
impl IntoResponseParts for HxReplaceUrl {
    type Error = <TypedHeader<Self> as IntoResponseParts>::Error;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        TypedHeader(self).into_response_parts(res)
    }
}

#[cfg(feature = "axum")]
impl IntoResponse for HxReplaceUrl {
    fn into_response(self) -> Response {
        TypedHeader(self).into_response()
    }
}

impl Header for HxReplaceUrl {
    fn name() -> &'static HeaderName {
        &HX_REPLACE_URL
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
                .expect("invalid value for HX-Replace-URL"),
        ));
    }
}
