use headers_core::Error;
use http::HeaderValue;
use serde::{Deserialize, Serialize};

const INNER_HTML: HeaderValue = HeaderValue::from_static("innerHTML");
const OUTER_HTML: HeaderValue = HeaderValue::from_static("outerHTML");
const TEXT_CONTENT: HeaderValue = HeaderValue::from_static("textContent");
const BEFORE_BEGIN: HeaderValue = HeaderValue::from_static("beforebegin");
const AFTER_BEGIN: HeaderValue = HeaderValue::from_static("afterbegin");
const BEFORE_END: HeaderValue = HeaderValue::from_static("beforeend");
const AFTER_END: HeaderValue = HeaderValue::from_static("afterend");
const DELETE: HeaderValue = HeaderValue::from_static("delete");
const NONE: HeaderValue = HeaderValue::from_static("none");

/// Allows you to specify how the response will be swapped in relative to the target of an AJAX request.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SwapOption {
    /// Replace the inner html of the target element.
    #[default]
    #[serde(rename = "innerHTML")]
    InnerHtml,
    /// Replace the entire target element with the response.
    #[serde(rename = "outerHTML")]
    OuterHtml,
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
    /// Deletes the target element regardless of the response.
    #[serde(rename = "delete")]
    Delete,
    /// Does not append content from response (out of band items will still be processed).
    #[serde(rename = "none")]
    None,
}

impl SwapOption {
    pub(crate) fn to_header_value(self) -> HeaderValue {
        match self {
            Self::InnerHtml => INNER_HTML.clone(),
            Self::OuterHtml => OUTER_HTML.clone(),
            Self::TextContent => TEXT_CONTENT.clone(),
            Self::BeforeBegin => BEFORE_BEGIN.clone(),
            Self::AfterBegin => AFTER_BEGIN.clone(),
            Self::BeforeEnd => BEFORE_END.clone(),
            Self::AfterEnd => AFTER_END.clone(),
            Self::Delete => DELETE.clone(),
            Self::None => NONE.clone(),
        }
    }

    pub(crate) fn from_header_value(header_value: &HeaderValue) -> Result<Self, Error> {
        match header_value {
            v if v == INNER_HTML => Ok(Self::InnerHtml),
            v if v == OUTER_HTML => Ok(Self::OuterHtml),
            v if v == TEXT_CONTENT => Ok(Self::TextContent),
            v if v == BEFORE_BEGIN => Ok(Self::BeforeBegin),
            v if v == AFTER_BEGIN => Ok(Self::AfterBegin),
            v if v == BEFORE_END => Ok(Self::BeforeEnd),
            v if v == AFTER_END => Ok(Self::AfterEnd),
            v if v == DELETE => Ok(Self::Delete),
            v if v == NONE => Ok(Self::None),
            _ => Err(Error::invalid()),
        }
    }
}
