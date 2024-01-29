//! # Overview
//!
//! `tower-helmet` helps you secure your tower server by setting various HTTP headers. _It's not a
//! silver bullet_, but it can help!
//!
//! You can find a list of all available headers under the [header] module. By default (with
//! [HelmetLayer::with_defaults]) **all of them** are enabled. Please take a good look at
//! [ContentSecurityPolicy]. Most of the time you will need to adapt this one to your needs.
//!
//! # Examples
//!
//! ```
//! use std::collections::HashMap;
//!
//! use tower_helmet::header::{ContentSecurityPolicy, ExpectCt, XFrameOptions};
//! use tower_helmet::HelmetLayer;
//!
//! // default layer with all security headers active
//! let layer = HelmetLayer::with_defaults();
//!
//! // default layer with csp customizations applied
//! let mut directives = HashMap::new();
//! directives.insert("default-src", vec!["'self'", "https://example.com"]);
//! directives.insert("img-src", vec!["'self'", "data:", "https://example.com"]);
//! directives.insert(
//!     "script-src",
//!     vec!["'self'", "'unsafe-inline'", "https://example.com"],
//! );
//! let csp = ContentSecurityPolicy {
//!     directives,
//!     ..Default::default()
//! };
//!
//! let layer = HelmetLayer::with_defaults().enable(csp);
//!
//! // completely blank layer, selectively enable and add headers
//! let layer = HelmetLayer::blank()
//!     .enable(XFrameOptions::SameOrigin)
//!     .enable(ExpectCt::default());
//! ```
pub mod header;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::ready;
use http::header::{AsHeaderName, HeaderName, InvalidHeaderValue};
use http::{HeaderMap, HeaderValue, Request, Response};
use pin_project_lite::pin_project;
use tower_layer::Layer;
use tower_service::Service;

use crate::header::{
    ContentSecurityPolicy, CrossOriginEmbedderPolicy, CrossOriginOpenerPolicy,
    CrossOriginResourcePolicy, ExpectCt, OriginAgentCluster, ReferrerPolicy,
    StrictTransportSecurity, XContentTypeOptions, XDnsPrefetchControl, XDownloadOptions,
    XFrameOptions, XPermittedCrossDomainPolicies, XXSSProtection,
};

pub trait IntoHeader {
    fn header_name(&self) -> HeaderName;
    fn header_value(&self) -> Result<HeaderValue, InvalidHeaderValue>;
}

/// HelmetLayer
#[derive(Debug, Clone)]
pub struct HelmetLayer {
    headers: HeaderMap,
}

impl HelmetLayer {
    /// Helmet without any headers added in by default. See [`enable`] for enabling headers.
    pub fn blank() -> Self {
        Self {
            headers: HeaderMap::new(),
        }
    }

    /// Helmet with most of the headers already added with the base configuration.
    #[allow(clippy::default_constructed_unit_structs)]
    pub fn with_defaults() -> Self {
        let mut layer = Self::blank();
        layer
            .enable(ContentSecurityPolicy::default())
            .enable(CrossOriginEmbedderPolicy::default())
            .enable(CrossOriginOpenerPolicy::default())
            .enable(CrossOriginResourcePolicy::default())
            .enable(ExpectCt::default())
            .enable(OriginAgentCluster::default())
            .enable(ReferrerPolicy::default())
            .enable(StrictTransportSecurity::default())
            .enable(XContentTypeOptions::default())
            .enable(XDnsPrefetchControl::default())
            .enable(XDownloadOptions::default())
            .enable(XFrameOptions::default())
            .enable(XPermittedCrossDomainPolicies::default())
            .enable(XXSSProtection::default());

        layer
    }

    pub fn enable(&mut self, h: impl IntoHeader) -> &mut Self {
        self.headers
            .insert(h.header_name(), h.header_value().unwrap());
        self
    }

    pub fn remove<K>(&mut self, key: K) -> &mut Self
    where
        K: AsHeaderName,
    {
        self.headers.remove(key);
        self
    }
}

impl<S> Layer<S> for HelmetLayer {
    type Service = HelmetService<S>;

    fn layer(&self, service: S) -> Self::Service {
        HelmetService {
            inner: service,
            headers: self.headers.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HelmetService<S> {
    inner: S,
    headers: HeaderMap,
}

impl<ReqBody, ResBody, S> Service<Request<ReqBody>> for HelmetService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<ReqBody>) -> Self::Future {
        ResponseFuture {
            future: self.inner.call(request),
            headers: self.headers.clone(),
        }
    }
}

pin_project! {
    /// Response future for [`HelmetService`].
    #[derive(Debug)]
    pub struct ResponseFuture<F> {
        #[pin]
        future: F,

        headers: HeaderMap,
    }
}

impl<F, ResBody, E> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response<ResBody>, E>>,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let mut res: Response<ResBody> = ready!(this.future.poll(cx)?);
        let headers = res.headers_mut();

        for (name, value) in this.headers {
            headers.insert(name, value.clone());
        }

        Poll::Ready(Ok(res))
    }
}
