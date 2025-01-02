use std::{iter::once, str::FromStr};

#[cfg(feature = "axum")]
use axum_core::response::{IntoResponse, IntoResponseParts, Response, ResponseParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header};
use http::{HeaderMap, HeaderName, HeaderValue, Uri};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::util::{iter::IterExt, uri::UriExt};

use super::options::{LocationOptions, SwapOption};

static HX_LOCATION: HeaderName = HeaderName::from_static("hx-location");

/// Allows you to do a client-side redirect that does not do a full page reload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HxLocation {
    #[serde(with = "http_serde::uri")]
    path: Uri,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    options: Option<LocationOptions>,
}

impl HxLocation {
    /// Create a new `HxLocation`.
    pub fn new(path: Uri) -> Self {
        Self {
            path,
            options: None,
        }
    }

    /// Specify options for the `HxLocation`.
    pub fn with_options(mut self, options: LocationOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// Get the path of the `HxLocation`.
    pub fn path(&self) -> &Uri {
        &self.path
    }

    /// Get the source element of the request.
    pub fn source(&self) -> Option<&str> {
        self.options.as_ref().and_then(|o| o.source())
    }

    /// Get the event that “triggered” the request.
    pub fn event(&self) -> Option<&str> {
        self.options.as_ref().and_then(|o| o.event())
    }

    /// Get the callback that will handle the response HTML.
    pub fn handler(&self) -> Option<&str> {
        self.options.as_ref().and_then(|o| o.handler())
    }

    /// Get the target to swap the response into.
    pub fn target(&self) -> Option<&str> {
        self.options.as_ref().and_then(|o| o.target())
    }

    /// Get how the response will be swapped in relative to the target.
    pub fn swap(&self) -> Option<SwapOption> {
        self.options.as_ref().and_then(|o| o.swap())
    }

    /// Get the values to submit with the request.
    pub fn values(&self) -> Option<&Value> {
        self.options.as_ref().and_then(|o| o.values())
    }

    /// Get the headers to submit with the request.
    pub fn headers(&self) -> Option<&HeaderMap> {
        self.options.as_ref().and_then(|o| o.headers())
    }

    /// Get the CSS selector that allows you to choose which part of the response is used to be swapped in.
    pub fn select(&self) -> Option<&str> {
        self.options.as_ref().and_then(|o| o.select())
    }
}

impl From<Uri> for HxLocation {
    fn from(path: Uri) -> Self {
        Self::new(path)
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponseParts for HxLocation {
    type Error = <TypedHeader<Self> as IntoResponseParts>::Error;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        TypedHeader(self).into_response_parts(res)
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponse for HxLocation {
    fn into_response(self) -> Response {
        TypedHeader(self).into_response()
    }
}

impl Header for HxLocation {
    fn name() -> &'static HeaderName {
        &HX_LOCATION
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values.just_one().ok_or_else(Error::invalid)?;
        let json_value: Value =
            serde_json::from_slice(value.as_bytes()).map_err(|_| Error::invalid())?;

        if json_value.is_string() {
            let path = Uri::from_str(json_value.as_str().unwrap()).map_err(|_| Error::invalid())?;
            return Ok(Self::new(path));
        }

        serde_json::from_value(json_value).map_err(|_| Error::invalid())
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        if self.options.is_none() {
            values.extend(once(
                HeaderValue::from_uri(&self.path).expect("invalid value for HX-Location"),
            ));
        } else {
            let value = serde_json::to_string(self).expect("invalid value for HX-Location");
            values.extend(once(
                HeaderValue::from_str(&value).expect("invalid value for HX-Location"),
            ));
        }
    }
}
