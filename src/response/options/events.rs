use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use http::HeaderValue;
use serde::Serialize;
use serde_json::Value;

/// A list of events that can be triggered using `HxTrigger`.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Events<'a>(Vec<Event<'a>>);

impl Events<'_> {
    /// Create a new `Events`.
    pub fn new() -> Self {
        Default::default()
    }

    /// Create a new `Events` with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    fn has_data(&self) -> bool {
        self.0.iter().any(|event| event.data.is_some())
    }

    pub(crate) fn to_header_value(&self) -> Option<HeaderValue> {
        if self.0.is_empty() {
            return Some(HeaderValue::from_static(""));
        }

        if self.has_data() {
            let mut map = BTreeMap::new();

            for event in &self.0 {
                let value = event.data.as_ref().unwrap_or(&Value::Null);
                map.insert(event.name, value);
            }

            let value = serde_json::to_string(&map).ok()?;

            Some(HeaderValue::from_str(&value).ok()?)
        } else {
            let value = self
                .0
                .iter()
                .map(|event| event.name)
                .collect::<Vec<_>>()
                .join(",");

            Some(HeaderValue::from_str(&value).ok()?)
        }
    }
}

impl<'a> Deref for Events<'a> {
    type Target = Vec<Event<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Events<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// An event that can be triggered using `HxTrigger`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Event<'a> {
    name: &'a str,
    data: Option<Value>,
}

impl<'a> Event<'a> {
    /// Create a new `Event` with the given name.
    pub fn new(name: &'a str) -> Self {
        Self { name, data: None }
    }

    /// Create a new `Event` with the given name and data.
    pub fn new_with_data<T>(name: &'a str, data: T) -> Result<Self, serde_json::Error>
    where
        T: Serialize,
    {
        Ok(Self {
            name,
            data: Some(serde_json::to_value(data)?),
        })
    }

    /// Get the name of the event.
    pub fn name(&self) -> &str {
        self.name
    }

    /// Get the data associated with the event.
    pub fn data(&self) -> Option<&Value> {
        self.data.as_ref()
    }
}
