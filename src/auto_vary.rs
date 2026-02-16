use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, ready},
};

use axum_core::{extract::Request, response::Response};
use http::{HeaderValue, header::VARY, request::Parts};
use pin_project_lite::pin_project;
use tower_layer::Layer;
use tower_service::Service;

const HX_BOOSTED: HeaderValue = HeaderValue::from_static("hx-boosted");
const HX_CURRENT_URL: HeaderValue = HeaderValue::from_static("hx-current-url");
const HX_HISTORY_RESTORE_REQUEST: HeaderValue =
    HeaderValue::from_static("hx-history-restore-request");
const HX_REQUEST: HeaderValue = HeaderValue::from_static("hx-request");
const HX_REQUEST_TYPE: HeaderValue = HeaderValue::from_static("hx-request-type");
const HX_SOURCE: HeaderValue = HeaderValue::from_static("hx-source");
const HX_TARGET: HeaderValue = HeaderValue::from_static("hx-target");

#[derive(Debug, Clone, Copy)]
pub struct HxRequestHeaderSet(u8);

impl HxRequestHeaderSet {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn add(&mut self, header: HxRequestHeader) {
        self.0 |= header.mask();
    }

    pub fn add_to_response(&self, response: &mut Response) {
        for hx_request_header in HxRequestHeader::iter() {
            if self.0 & hx_request_header.mask() != 0 {
                hx_request_header.add_to_response(response);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HxRequestHeader {
    Boosted,
    CurrentUrl,
    HistoryRestoreRequest,
    Request,
    RequestType,
    Source,
    Target,
}

impl HxRequestHeader {
    pub fn iter() -> impl IntoIterator<Item = Self> {
        [
            HxRequestHeader::Boosted,
            HxRequestHeader::CurrentUrl,
            HxRequestHeader::HistoryRestoreRequest,
            HxRequestHeader::Request,
            HxRequestHeader::RequestType,
            HxRequestHeader::Source,
            HxRequestHeader::Target,
        ]
    }

    pub fn mask(&self) -> u8 {
        match self {
            HxRequestHeader::Boosted => 1 << 0,
            HxRequestHeader::CurrentUrl => 1 << 1,
            HxRequestHeader::HistoryRestoreRequest => 1 << 2,
            HxRequestHeader::Request => 1 << 3,
            HxRequestHeader::RequestType => 1 << 4,
            HxRequestHeader::Source => 1 << 5,
            HxRequestHeader::Target => 1 << 6,
        }
    }

    pub fn value(self) -> HeaderValue {
        match self {
            HxRequestHeader::Boosted => HX_BOOSTED,
            HxRequestHeader::CurrentUrl => HX_CURRENT_URL,
            HxRequestHeader::HistoryRestoreRequest => HX_HISTORY_RESTORE_REQUEST,
            HxRequestHeader::Request => HX_REQUEST,
            HxRequestHeader::RequestType => HX_REQUEST_TYPE,
            HxRequestHeader::Source => HX_SOURCE,
            HxRequestHeader::Target => HX_TARGET,
        }
    }

    pub fn add_to_response(self, response: &mut Response) {
        response.headers_mut().insert(VARY, self.value());
    }
}

/// A layer that automatically adds the `Vary` header to responses based on the extracted HTMX headers. Read more about
/// caching in HTMX [here](https://htmx.org/docs/#caching).
#[derive(Debug, Clone)]
pub struct AutoVaryLayer;

impl<S> Layer<S> for AutoVaryLayer {
    type Service = AutoVary<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AutoVary { inner }
    }
}

/// A service that automatically adds the `Vary` header to responses based on the extracted HTMX headers. Read more
/// about caching in HTMX [here](https://htmx.org/docs/#caching).
#[derive(Debug, Clone)]
pub struct AutoVary<S> {
    inner: S,
}

impl<S> Service<Request> for AutoVary<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;

    type Error = S::Error;

    type Future = AutoVaryResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        let set = Arc::new(Mutex::new(HxRequestHeaderSet::new()));
        req.extensions_mut().insert(set.clone());

        let fut = self.inner.call(req);
        AutoVaryResponseFuture { fut, set }
    }
}

pin_project! {
    pub struct AutoVaryResponseFuture<F> {
        #[pin]
        fut: F,
        set: Arc<Mutex<HxRequestHeaderSet>>,
    }
}

impl<F, E> Future for AutoVaryResponseFuture<F>
where
    F: Future<Output = Result<Response, E>>,
{
    type Output = Result<Response, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let mut response = ready!(this.fut.poll(cx));

        if let Ok(ref mut response) = response {
            if let Ok(lock) = this.set.lock() {
                lock.add_to_response(response);
            }
        }

        Poll::Ready(response)
    }
}

pub trait AutoVaryAdd {
    fn auto_vary_add(self, header: HxRequestHeader);
}

impl AutoVaryAdd for &mut Parts {
    fn auto_vary_add(self, header: HxRequestHeader) {
        if let Some(set) = self.extensions.get_mut::<Arc<Mutex<HxRequestHeaderSet>>>() {
            if let Ok(mut lock) = set.lock() {
                lock.add(header);
            }
        }
    }
}
