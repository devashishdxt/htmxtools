use std::{iter::once, ops::Deref};

#[cfg(feature = "axum")]
use axum_core::response::{IntoResponse, IntoResponseParts, Response, ResponseParts};
#[cfg(feature = "axum")]
use axum_extra::TypedHeader;
use headers_core::{Error, Header};
use http::{HeaderName, HeaderValue};

use crate::util::iter::IterExt;

use super::options::SwapOption;

static HX_RESWAP: HeaderName = HeaderName::from_static("hx-reswap");

/// Allows you to specify how the response will be swapped. See [`SwapOption`] for possible values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HxReswap(SwapOption);

impl HxReswap {
    /// Replace the inner html of the target element.
    pub const fn inner_html() -> Self {
        Self(SwapOption::InnerHtml)
    }

    /// Replace the entire target element with the response.
    pub const fn outer_html() -> Self {
        Self(SwapOption::OuterHtml)
    }

    /// Replace the text content of the target element, without parsing the response as HTML.
    pub const fn text_content() -> Self {
        Self(SwapOption::TextContent)
    }

    /// Insert the response before the target element.
    pub const fn before_begin() -> Self {
        Self(SwapOption::BeforeBegin)
    }

    /// Insert the response before the first child of the target element.
    pub const fn after_begin() -> Self {
        Self(SwapOption::AfterBegin)
    }

    /// Insert the response after the last child of the target element.
    pub const fn before_end() -> Self {
        Self(SwapOption::BeforeEnd)
    }

    /// Insert the response after the target element.
    pub const fn after_end() -> Self {
        Self(SwapOption::AfterEnd)
    }

    /// Deletes the target element regardless of the response.
    pub const fn delete() -> Self {
        Self(SwapOption::Delete)
    }

    /// Does not append content from response (out of band items will still be processed).
    pub const fn none() -> Self {
        Self(SwapOption::None)
    }

    /// Get the value of the `HxReswap`.
    pub fn swap_option(&self) -> SwapOption {
        self.0
    }
}

impl Deref for HxReswap {
    type Target = SwapOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<SwapOption> for HxReswap {
    fn from(swap_option: SwapOption) -> Self {
        Self(swap_option)
    }
}

impl From<HxReswap> for SwapOption {
    fn from(hx_reswap: HxReswap) -> Self {
        hx_reswap.0
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

    fn decode<'i, I>(values: &mut I) -> Result<Self, Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values.just_one().ok_or_else(Error::invalid)?;
        let swap_option = SwapOption::from_header_value(value)?;

        Ok(Self(swap_option))
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        values.extend(once(self.0.to_header_value()));
    }
}
