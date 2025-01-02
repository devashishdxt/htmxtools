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

static HX_TRIGGER: HeaderName = HeaderName::from_static("hx-trigger");

/// Allows you to trigger client-side events.
pub struct HxTrigger<'a>(Events<'a>);

impl<'a> HxTrigger<'a> {
    /// Create a new `HxTrigger` with the given event name.
    pub fn event(name: &'a str) -> Self {
        let mut events = Events::with_capacity(1);
        events.push(Event::new(name));
        Self(events)
    }

    /// Create a new `HxTrigger` with the given event names.
    pub fn events(names: &[&'a str]) -> Self {
        let mut events = Events::with_capacity(names.len());
        for name in names {
            events.push(Event::new(name));
        }
        Self(events)
    }

    /// Create a new `HxTrigger` with the given event name and data.
    pub fn event_with_data<T>(name: &'a str, data: T) -> Result<Self, serde_json::Error>
    where
        T: Serialize,
    {
        let mut events = Events::with_capacity(1);
        events.push(Event::new_with_data(name, data)?);
        Ok(Self(events))
    }
}

impl<'a> Deref for HxTrigger<'a> {
    type Target = Events<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HxTrigger<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> From<Events<'a>> for HxTrigger<'a> {
    fn from(events: Events<'a>) -> Self {
        Self(events)
    }
}

#[cfg(feature = "axum")]
impl IntoResponseParts for HxTrigger<'_> {
    type Error = <TypedHeader<Self> as IntoResponseParts>::Error;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        TypedHeader(self).into_response_parts(res)
    }
}

#[cfg(feature = "axum")]
impl IntoResponse for HxTrigger<'_> {
    fn into_response(self) -> Response {
        TypedHeader(self).into_response()
    }
}

impl Header for HxTrigger<'_> {
    fn name() -> &'static HeaderName {
        &HX_TRIGGER
    }

    fn decode<'i, I>(_: &mut I) -> Result<Self, Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        // This is a response header, so we don't need to decode it. Also, `HX-Trigger` is seperately decoded in
        // `crate::request::HxTrigger`.
        Err(Error::invalid())
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        let value = self
            .0
            .to_header_value()
            .expect("invalid value for HX-Trigger");

        values.extend(once(value));
    }
}
