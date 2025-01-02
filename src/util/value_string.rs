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

    pub(crate) fn from_string(src: String) -> Option<Self> {
        // A valid `str` (the argument)...
        let bytes = Bytes::from(src);
        HeaderValue::from_maybe_shared(bytes).ok().map(Self)
    }

    pub(crate) const fn from_static(src: &'static str) -> HeaderValueString {
        // A valid `str` (the argument)...
        Self(HeaderValue::from_static(src))
    }

    pub(crate) fn as_str(&self) -> &str {
        // HeaderValueString is only created from HeaderValues
        // that have validated they are also UTF-8 strings.
        unsafe { std::str::from_utf8_unchecked(self.0.as_bytes()) }
    }

    pub(crate) fn as_header_value(&self) -> &HeaderValue {
        &self.0
    }
}
