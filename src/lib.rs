//! A lightweight Rust crate for working with HTMX headers, specifically designed to integrate seamlessly with `axum`.
//!
//! HTMX is a library that enhances HTML with AJAX capabilities, and it relies heavily on custom headers to enable
//! dynamic interactions. The `htmx-headers`` crate simplifies handling these headers in Rust, enabling you to easily
//! extract, process, and respond with HTMX-related data.
//!
//! # Features
//!
//! - **Request Extractors**: Easily extract HTMX headers from incoming requests.
//! - **Response Builders**: Conveniently build responses with HTMX headers.
//! - **Axum Integration**: Built on top of `typed-headers` in `axum-extra` for seamless integration with the `axum`.
//! - **Auto Vary**: Correctly handle response caching by automatically add the `Vary` header to responses based on the
//!   extracted HTMX headers.
//!
//! # Usage
//!
//! To use `htmx-headers`, run the following command in your project directory:
//!
//! ```bash
//! cargo add htmx-headers
//! ```
//!
//! If you don't want to use `axum`, you can still use the `htmx-headers` crate with other web frameworks. The crate
//! implements `headers::Header` trait for all HTMX headers, so you can easily extract and build headers in any
//! framework that supports `headers::Header`.
//!
//! To use `htmx-headers` without `axum`, run the following command in your project directory:
//!
//! ```bash
//! cargo add htmx-headers --no-default-features
//! ```
//!
//! ## Request Extractors
//!
//! To extract HTMX headers from incoming requests in `axum`, you can directly use headers in [`crate::request`] in your
//! handler functions as they implement `FromRequestParts` and `OptionalFromRequestParts` traits.
//!
//! Here's an example of extracting the `hx-request` header:
//!
//! ```rust,no_run
//! use axum_core::response::IntoResponse;
//! use htmx_headers::request::HxRequest;
//!
//! async fn handler(hx_request: Option<HxRequest>) -> impl IntoResponse {
//!     if hx_request.is_some() {
//!         "This is an HTMX request"
//!     } else {
//!         "This is not an HTMX request"
//!     }
//! }
//! ```
//!
//! Here's another example of extracting the `hx-target` header:
//!
//! ```rust,no_run
//! use axum_core::response::IntoResponse;
//! use htmx_headers::request::HxTarget;
//!
//! async fn handler(hx_target: HxTarget) -> impl IntoResponse {
//!     format!("The target is: {}", hx_target.as_str())
//! }
//!
//! async fn another_handler(hx_target: Option<HxTarget>) -> impl IntoResponse {
//!     match hx_target {
//!         Some(target) => format!("The target is: {}", target.as_str()),
//!         None => "No target specified".to_string(),
//!     }
//! }
//! ```
//!
//! ## Response Builders
//!
//! To build responses with HTMX headers in `axum`, you can use headers in [`crate::response`] in your handler functions
//! as they implement `IntoResponseParts` and `IntoResponse` traits.
//!
//! Here's an example of building a response with the `hx-push-url` header:
//!
//! ```rust,no_run
//! use axum_core::response::IntoResponse;
//! use htmx_headers::response::HxPushUrl;
//! use http::Uri;
//!
//! async fn handler() -> impl IntoResponse {
//!     HxPushUrl::url(Uri::from_static("/new-url"))
//! }
//! ```
//!
//! Here's another example of building a response with `hx-retarget` and `hx-reswap` header:
//!
//! ```rust,no_run
//! use axum_core::response::IntoResponse;
//! use htmx_headers::response::{HxReswap, HxRetarget};
//!
//! async fn handler() -> impl IntoResponse {
//!     (HxReswap::inner_html(), HxRetarget::from_static("#body"), "<div></div>")
//! }
//! ```
//!
//! ## Auto Vary
//!
//! To automatically add the `Vary` header to responses based on the extracted HTMX headers in `axum`, you can use the
//! `AutoVaryLayer` in [`crate::auto_vary`].
//!
//! To use the `AutoVaryLayer`, you need to enable the `auto-vary` feature in your `Cargo.toml`.
//!
//! Here's an example of using the `AutoVaryLayer`:
//!
//! ```rust,ignore
//! use axum::Router;
//! use htmx_headers::auto_vary::AutoVaryLayer;
//!
//! fn app() -> Router {
//!     Router::new().layer(AutoVaryLayer)
//! }
//! ```
#[cfg(feature = "auto-vary")]
#[cfg_attr(docsrs, doc(cfg(feature = "auto-vary")))]
pub mod auto_vary;
pub mod request;
pub mod response;
mod util;
