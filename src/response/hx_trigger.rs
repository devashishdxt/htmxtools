use std::{collections::BTreeMap, iter::once};

#[cfg(feature = "axum")]
use axum_core::response::{IntoResponse, IntoResponseParts, Response, ResponseParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header};
use http::{HeaderName, HeaderValue};
use serde_json::Value;

static HX_TRIGGER: HeaderName = HeaderName::from_static("hx-trigger");

/// Allows you to trigger client-side events.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HxTrigger(Events);

impl HxTrigger {
    /// Creates a new `HxTrigger` instance.
    pub fn new() -> Self {
        Self(Events::new())
    }

    /// Creates a new `HxTrigger` instance with a specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Events::with_capacity(capacity))
    }

    /// Pushes an event with no data.
    pub fn push_event(&mut self, name: String) -> &mut Self {
        self.0.events.push(Event { name, data: None });
        self
    }

    /// Pushes an event with data.
    pub fn push_event_data(&mut self, name: String, data: Value) -> &mut Self {
        self.0.events.push(Event {
            name,
            data: Some(data),
        });
        self
    }

    /// Pushes an event with no data.
    pub fn with_event(mut self, name: String) -> Self {
        self.0.events.push(Event { name, data: None });
        self
    }

    /// Pushes an event with data.
    pub fn with_event_data(mut self, name: String, data: Value) -> Self {
        self.0.events.push(Event {
            name,
            data: Some(data),
        });
        self
    }

    pub(crate) fn to_header_value(&self) -> Option<HeaderValue> {
        self.0.to_header_value()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Events {
    events: Vec<Event>,
}

impl Events {
    fn new() -> Self {
        Self { events: Vec::new() }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            events: Vec::with_capacity(capacity),
        }
    }

    fn has_data(&self) -> bool {
        self.events.iter().any(|event| event.has_data())
    }

    fn to_header_value(&self) -> Option<HeaderValue> {
        if self.events.is_empty() {
            return Some(HeaderValue::from_static(""));
        }

        if self.has_data() {
            let mut map = BTreeMap::new();

            for event in &self.events {
                let value = event.data.as_ref().unwrap_or(&Value::Null);
                map.insert(&event.name, value);
            }

            let value = serde_json::to_string(&map).ok()?;

            Some(HeaderValue::from_str(&value).ok()?)
        } else {
            let value = self
                .events
                .iter()
                .map(|event| event.name.as_str())
                .collect::<Vec<_>>()
                .join(",");

            Some(HeaderValue::from_str(&value).ok()?)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Event {
    name: String,
    data: Option<serde_json::Value>,
}

impl Event {
    fn has_data(&self) -> bool {
        self.data.is_some()
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponseParts for HxTrigger {
    type Error = <TypedHeader<Self> as IntoResponseParts>::Error;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        TypedHeader(self).into_response_parts(res)
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponse for HxTrigger {
    fn into_response(self) -> Response {
        TypedHeader(self).into_response()
    }
}

impl Header for HxTrigger {
    fn name() -> &'static HeaderName {
        &HX_TRIGGER
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
