use std::iter::once;

#[cfg(feature = "axum")]
use axum_core::response::{IntoResponse, IntoResponseParts, Response, ResponseParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header};
use http::{HeaderName, HeaderValue};
use serde_json::Value;

use crate::response::HxTrigger;

static HX_TRIGGER_AFTER_SETTLE: HeaderName = HeaderName::from_static("hx-trigger-after-settle");

/// Allows you to trigger client-side events after the settle step.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HxTriggerAfterSettle(HxTrigger);

impl HxTriggerAfterSettle {
    /// Creates a new `HxTrigger` instance.
    pub fn new() -> Self {
        Self(HxTrigger::new())
    }

    /// Creates a new `HxTrigger` instance with a specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HxTrigger::with_capacity(capacity))
    }

    /// Pushes an event with no data.
    pub fn push_event(&mut self, name: String) -> &mut Self {
        self.0.push_event(name);
        self
    }

    /// Pushes an event with data.
    pub fn push_event_data(&mut self, name: String, data: Value) -> &mut Self {
        self.0.push_event_data(name, data);
        self
    }

    /// Pushes an event with no data.
    pub fn with_event(self, name: String) -> Self {
        Self(self.0.with_event(name))
    }

    /// Pushes an event with data.
    pub fn with_event_data(self, name: String, data: Value) -> Self {
        Self(self.0.with_event_data(name, data))
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponseParts for HxTriggerAfterSettle {
    type Error = <TypedHeader<Self> as IntoResponseParts>::Error;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        TypedHeader(self).into_response_parts(res)
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponse for HxTriggerAfterSettle {
    fn into_response(self) -> Response {
        TypedHeader(self).into_response()
    }
}

impl Header for HxTriggerAfterSettle {
    fn name() -> &'static HeaderName {
        &HX_TRIGGER_AFTER_SETTLE
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
        if let Some(value) = self.0.to_header_value() {
            values.extend(once(value));
        }
    }
}
