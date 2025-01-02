use http::HeaderMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::SwapOption;

/// Options for the [`HxLocation`](crate::response::HxLocation) header.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocationOptions {
    /// The source element of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<String>,
    /// An event that “triggered” the request
    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<String>,
    /// A callback that will handle the response HTML.
    #[serde(skip_serializing_if = "Option::is_none")]
    handler: Option<String>,
    /// The target to swap the response into.
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<String>,
    /// How the response will be swapped in relative to the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    swap: Option<SwapOption>,
    /// Values to submit with the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<Value>,
    /// Headers to submit with the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "http_serde::option::header_map")]
    headers: Option<HeaderMap>,
    /// Allows you to select the content you want swapped from a response.
    #[serde(skip_serializing_if = "Option::is_none")]
    select: Option<String>,
}

impl LocationOptions {
    /// Create a new `LocationOptions`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Specify the source element of the request.
    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }

    /// Specify an event that “triggered” the request.
    pub fn with_event(mut self, event: String) -> Self {
        self.event = Some(event);
        self
    }

    /// Specify a callback that will handle the response HTML.
    pub fn with_handler(mut self, handler: String) -> Self {
        self.handler = Some(handler);
        self
    }

    /// Specify the target to swap the response into.
    pub fn with_target(mut self, target: String) -> Self {
        self.target = Some(target);
        self
    }

    /// Specify how the response will be swapped in relative to the target.
    pub fn with_swap(mut self, swap: SwapOption) -> Self {
        self.swap = Some(swap);
        self
    }

    /// Specify values to submit with the request.
    pub fn with_values<T>(mut self, values: T) -> Result<Self, serde_json::Error>
    where
        T: Serialize,
    {
        self.values = Some(serde_json::to_value(values)?);
        Ok(self)
    }

    /// Specify headers to submit with the request.
    pub fn with_headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Specify a CSS selector that allows you to choose which part of the response is used to be swapped in.
    pub fn with_select(mut self, select: String) -> Self {
        self.select = Some(select);
        self
    }

    /// Get the source element of the request.
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    /// Get the event that “triggered” the request.
    pub fn event(&self) -> Option<&str> {
        self.event.as_deref()
    }

    /// Get the callback that will handle the response HTML.
    pub fn handler(&self) -> Option<&str> {
        self.handler.as_deref()
    }

    /// Get the target to swap the response into.
    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    /// Get how the response will be swapped in relative to the target.
    pub fn swap(&self) -> Option<SwapOption> {
        self.swap
    }

    /// Get the values to submit with the request.
    pub fn values(&self) -> Option<&Value> {
        self.values.as_ref()
    }

    /// Get the headers to submit with the request.
    pub fn headers(&self) -> Option<&HeaderMap> {
        self.headers.as_ref()
    }

    /// Get the CSS selector that allows you to choose which part of the response is used to be swapped in.
    pub fn select(&self) -> Option<&str> {
        self.select.as_deref()
    }
}
