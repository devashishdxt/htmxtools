use bytes::Bytes;
use headers_core::Error;
use http::HeaderValue;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeaderValueString(HeaderValue);

impl HeaderValueString {
    pub(crate) fn try_from_header_value(value: &HeaderValue) -> Result<Self, Error> {
        if value.to_str().is_ok() {
            Ok(Self(value.clone()))
        } else {
            Err(Error::invalid())
        }
    }

    pub(crate) const fn from_static(src: &'static str) -> Self {
        Self(HeaderValue::from_static(src))
    }

    pub(crate) fn from_str(src: &str) -> Option<Self> {
        HeaderValue::from_str(src).ok().map(Self)
    }

    pub(crate) fn from_string(src: String) -> Option<Self> {
        let bytes = Bytes::from(src);
        HeaderValue::from_maybe_shared(bytes).ok().map(Self)
    }

    pub(crate) fn as_str(&self) -> &str {
        // SAFETY: HeaderValueString is only created from HeaderValues that have been
        // validated to be valid UTF-8 strings. All constructors (`try_from_header_value`,
        // `from_string`, and `from_static`) ensure the HeaderValue can be represented as
        // a valid UTF-8 string before wrapping it in HeaderValueString.
        unsafe { std::str::from_utf8_unchecked(self.0.as_bytes()) }
    }

    pub(crate) fn as_header_value(&self) -> &HeaderValue {
        &self.0
    }
}
