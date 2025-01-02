use std::{
    collections::BTreeSet,
    task::{Context, Poll},
};

use axum_core::{extract::Request, response::Response};
use futures_core::future::BoxFuture;
use http::request::Parts;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tower_layer::Layer;
use tower_service::Service;

const NUM_HX_REQUEST_HEADERS: usize = 8;

pub(crate) enum HxRequestHeader {
    Boosted,
    CurrentUrl,
    HistoryRestoreRequest,
    Prompt,
    Request,
    Target,
    TriggerName,
    Trigger,
}

impl HxRequestHeader {
    fn as_str(&self) -> &'static str {
        match self {
            HxRequestHeader::Boosted => "HX-Boosted",
            HxRequestHeader::CurrentUrl => "HX-Current-URL",
            HxRequestHeader::HistoryRestoreRequest => "HX-History-Restore-Request",
            HxRequestHeader::Prompt => "HX-Prompt",
            HxRequestHeader::Request => "HX-Request",
            HxRequestHeader::Target => "HX-Target",
            HxRequestHeader::TriggerName => "HX-Trigger-Name",
            HxRequestHeader::Trigger => "HX-Trigger",
        }
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

    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        let (sender, mut receiver) = unbounded_channel::<HxRequestHeader>();
        req.extensions_mut().insert(sender);

        let fut = self.inner.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            let mut received_headers = Vec::with_capacity(NUM_HX_REQUEST_HEADERS);
            while let Some(header) = receiver.recv().await {
                received_headers.push(header.as_str());
            }

            if received_headers.is_empty() {
                return Ok(res);
            }

            let mut processed_headers = BTreeSet::new();

            for received_header in received_headers {
                if !processed_headers.contains(received_header) {
                    res.headers_mut().append(
                        http::header::VARY,
                        received_header.parse().expect("invalid htmx vary header"),
                    );
                    processed_headers.insert(received_header);
                }
            }

            Ok(res)
        })
    }
}

pub(crate) trait AutoVaryNotify {
    async fn auto_vary_notify(self, header: HxRequestHeader);
}

impl AutoVaryNotify for &mut Parts {
    async fn auto_vary_notify(self, header: HxRequestHeader) {
        if let Some(sender) = self
            .extensions
            .get_mut::<UnboundedSender<HxRequestHeader>>()
        {
            // ignore the error if the receiver is dropped
            let _ = sender.send(header);
        }
    }
}
