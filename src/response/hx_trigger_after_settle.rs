use std::{
    iter::once,
    ops::{Deref, DerefMut},
};

#[cfg(feature = "axum")]
use axum_core::response::{IntoResponse, IntoResponseParts, Response, ResponseParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header};
use http::{HeaderName, HeaderValue};
use serde::Serialize;

use super::options::{Event, Events};

static HX_TRIGGER_AFTER_SETTLE: HeaderName = HeaderName::from_static("hx-trigger-after-settle");

/// Allows you to trigger client-side events after the settle step.
pub struct HxTriggerAfterSettle<'a>(Events<'a>);

impl<'a> HxTriggerAfterSettle<'a> {
    /// Create a new `HxTriggerAfterSettle` with the given event name.
    pub fn event(name: &'a str) -> Self {
        let mut events = Events::with_capacity(1);
        events.push(Event::new(name));
        Self(events)
    }

    /// Create a new `HxTriggerAfterSettle` with the given event names.
    pub fn events(names: &[&'a str]) -> Self {
        let mut events = Events::with_capacity(names.len());
        for name in names {
            events.push(Event::new(name));
        }
        Self(events)
    }

    /// Create a new `HxTriggerAfterSettle` with the given event name and data.
    pub fn event_with_data<T>(name: &'a str, data: T) -> Result<Self, serde_json::Error>
    where
        T: Serialize,
    {
        let mut events = Events::with_capacity(1);
        events.push(Event::new_with_data(name, data)?);
        Ok(Self(events))
    }
}

impl<'a> Deref for HxTriggerAfterSettle<'a> {
    type Target = Events<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HxTriggerAfterSettle<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> From<Events<'a>> for HxTriggerAfterSettle<'a> {
    fn from(events: Events<'a>) -> Self {
        Self(events)
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponseParts for HxTriggerAfterSettle<'_> {
    type Error = <TypedHeader<Self> as IntoResponseParts>::Error;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        TypedHeader(self).into_response_parts(res)
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponse for HxTriggerAfterSettle<'_> {
    fn into_response(self) -> Response {
        TypedHeader(self).into_response()
    }
}

impl Header for HxTriggerAfterSettle<'_> {
    fn name() -> &'static HeaderName {
        &HX_TRIGGER_AFTER_SETTLE
    }

    fn decode<'i, I>(_: &mut I) -> Result<Self, Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        // This is a response header, so we don't need to decode it.
        Err(Error::invalid())
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        let value = self
            .0
            .to_header_value()
            .expect("invalid value for HX-Trigger-After-Settle");

        values.extend(once(value));
    }
}
