use std::iter::once;

#[cfg(feature = "axum")]
use axum_core::response::{IntoResponse, IntoResponseParts, Response, ResponseParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header};
use http::{HeaderName, HeaderValue};

const INNER_HTML: HeaderValue = HeaderValue::from_static("innerHTML");
const OUTER_HTML: HeaderValue = HeaderValue::from_static("outerHTML");
const INNER_MORPH: HeaderValue = HeaderValue::from_static("innerMorph");
const OUTER_MORPH: HeaderValue = HeaderValue::from_static("outerMorph");
const TEXT_CONTENT: HeaderValue = HeaderValue::from_static("textContent");
const BEFORE_BEGIN: HeaderValue = HeaderValue::from_static("beforebegin");
const AFTER_BEGIN: HeaderValue = HeaderValue::from_static("afterbegin");
const BEFORE_END: HeaderValue = HeaderValue::from_static("beforeend");
const AFTER_END: HeaderValue = HeaderValue::from_static("afterend");
const DELETE: HeaderValue = HeaderValue::from_static("delete");
const NONE: HeaderValue = HeaderValue::from_static("none");
const UPSERT: HeaderValue = HeaderValue::from_static("upsert");

static HX_RESWAP: HeaderName = HeaderName::from_static("hx-reswap");

/// Allows you to specify how the response will be swapped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HxReswap {
    /// Replace the inner html of the target element.
    InnerHtml,

    /// Replace the entire target element with the response.
    OuterHtml,

    /// Morphs the inner HTML of the target to the new content (see [Morphing](https://four.htmx.org/morphing/) for
    /// details).
    InnerMorph,

    /// Morphs the outer HTML of the target to the new content (see [Morphing](https://four.htmx.org/morphing/) for
    /// details)
    OuterMorph,

    /// Replace the text content of the target element, without parsing the response as HTML.
    TextContent,

    /// Insert the response before the target element.
    BeforeBegin,

    /// Insert the response before the first child of the target element.
    AfterBegin,

    /// Insert the response after the last child of the target element.
    BeforeEnd,

    /// Insert the response after the target element.
    AfterEnd,

    /// Delete the target element regardless of the response
    Delete,

    /// Does not append content from response (out of band items will still be processed).
    None,

    /// Updates existing elements by ID and inserts new ones (requires
    /// [upsert extension](https://four.htmx.org/extensions/upsert/)).
    Upsert,
}

impl HxReswap {
    fn to_header_value(self) -> HeaderValue {
        match self {
            HxReswap::InnerHtml => INNER_HTML,
            HxReswap::OuterHtml => OUTER_HTML,
            HxReswap::InnerMorph => INNER_MORPH,
            HxReswap::OuterMorph => OUTER_MORPH,
            HxReswap::TextContent => TEXT_CONTENT,
            HxReswap::BeforeBegin => BEFORE_BEGIN,
            HxReswap::AfterBegin => AFTER_BEGIN,
            HxReswap::BeforeEnd => BEFORE_END,
            HxReswap::AfterEnd => AFTER_END,
            HxReswap::Delete => DELETE,
            HxReswap::None => NONE,
            HxReswap::Upsert => UPSERT,
        }
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponseParts for HxReswap {
    type Error = <TypedHeader<Self> as IntoResponseParts>::Error;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        TypedHeader(self).into_response_parts(res)
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
impl IntoResponse for HxReswap {
    fn into_response(self) -> Response {
        TypedHeader(self).into_response()
    }
}

impl Header for HxReswap {
    fn name() -> &'static HeaderName {
        &HX_RESWAP
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
        values.extend(once(self.to_header_value()));
    }
}
