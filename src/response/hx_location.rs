use std::iter::once;

#[cfg(feature = "axum")]
use axum_core::response::{IntoResponse, IntoResponseParts, Response, ResponseParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header};
use http::{HeaderMap, HeaderName, HeaderValue, Uri};
use serde::Serialize;

use crate::util::uri::UriExt;

static HX_LOCATION: HeaderName = HeaderName::from_static("hx-location");

/// Allows you to do a client-side redirect that does not do a full page reload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HxLocation {
    /// The URL to navigate to.
    #[serde(with = "http_serde::uri")]
    pub path: Uri,

    /// CSS selector for swap target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// CSS selector for request source element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Swap strategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap: Option<HxLocationSwapOption>,

    /// CSS selector to pick from response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select: Option<String>,

    /// Push URL into browser history. Defaults to `true` — set `false` to suppress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push: Option<bool>,

    /// Extra request headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "http_serde::option::header_map")]
    pub headers: Option<HeaderMap>,

    /// Override body params.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<serde_json::Value>,
}

impl HxLocation {
    /// Creates a new `HxLocation` with the given path.
    pub fn new(path: Uri) -> Self {
        Self {
            path,
            target: None,
            source: None,
            swap: None,
            select: None,
            push: None,
            headers: None,
            values: None,
        }
    }

    /// Sets the CSS selector for swap target.
    pub fn with_target(mut self, target: String) -> Self {
        self.target = Some(target);
        self
    }

    /// Sets the CSS selector for request source element.
    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }

    /// Sets the swap strategy.
    pub fn with_swap(mut self, swap: HxLocationSwapOption) -> Self {
        self.swap = Some(swap);
        self
    }

    /// Sets the CSS selector to pick from response.
    pub fn with_select(mut self, select: String) -> Self {
        self.select = Some(select);
        self
    }

    /// Sets whether to push the new URL to the browser history.
    pub fn with_push(mut self, push: bool) -> Self {
        self.push = Some(push);
        self
    }

    /// Sets the headers to send with the request.
    pub fn with_headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Sets the body params to send with the request.
    pub fn with_values(mut self, values: serde_json::Value) -> Self {
        self.values = Some(values);
        self
    }

    fn is_options_none(&self) -> bool {
        self.target.is_none()
            && self.source.is_none()
            && self.swap.is_none()
            && self.select.is_none()
            && self.push.is_none()
            && self.headers.is_none()
            && self.values.is_none()
    }

    fn to_header_value(&self) -> Option<HeaderValue> {
        if self.is_options_none() {
            HeaderValue::from_uri(&self.path)
        } else {
            serde_json::to_string(self)
                .ok()
                .and_then(|value| HeaderValue::from_str(&value).ok())
        }
    }
}

/// Swap options for the `hx-location` header.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub enum HxLocationSwapOption {
    /// Replace the inner html of the target element.
    #[default]
    #[serde(rename = "innerHTML")]
    InnerHtml,

    /// Replace the entire target element with the response.
    #[serde(rename = "outerHTML")]
    OuterHtml,

    /// Morphs the inner HTML of the target to the new content (see [Morphing](https://four.htmx.org/morphing/) for
    /// details).
    #[serde(rename = "innerMorph")]
    InnerMorph,

    /// Morphs the outer HTML of the target to the new content (see [Morphing](https://four.htmx.org/morphing/) for
    /// details)
    #[serde(rename = "outerMorph")]
    OuterMorph,

    /// Replace the text content of the target element, without parsing the response as HTML.
    #[serde(rename = "textContent")]
    TextContent,

    /// Insert the response before the target element.
    #[serde(rename = "beforebegin")]
    BeforeBegin,

    /// Insert the response before the first child of the target element.
    #[serde(rename = "afterbegin")]
    AfterBegin,

    /// Insert the response after the last child of the target element.
    #[serde(rename = "beforeend")]
    BeforeEnd,

    /// Insert the response after the target element.
    #[serde(rename = "afterend")]
    AfterEnd,

    /// Delete the target element regardless of the response
    #[serde(rename = "delete")]
    Delete,

    /// Does not append content from response (out of band items will still be processed).
    #[serde(rename = "none")]
    None,

    /// Updates existing elements by ID and inserts new ones (requires
    /// [upsert extension](https://four.htmx.org/extensions/upsert/)).
    #[serde(rename = "upsert")]
    Upsert,
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

    fn decode<'i, I>(_: &mut I) -> Result<Self, Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        // This is a response header, so decoding it is not valid.
        Err(Error::invalid())
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        if let Some(value) = self.to_header_value() {
            values.extend(once(value));
        }
    }
}
