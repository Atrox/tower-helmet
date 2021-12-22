//! # Overview
//!
//! `tower-helmet` helps you secure your tower server by setting various HTTP headers. _It's not a silver bullet_, but it can help!
//!
//! You can find a list of all available headers under the [header] module. By default (with [HelmetLayer::default]) **all of them** are enabled.
//! Please take a good look at [ContentSecurityPolicy]. Most of the time you will need to adapt this one to your needs.
//!
//! # Examples
//!
//! ```
//! use tower_helmet::header::{ContentSecurityPolicy, ExpectCt, XFrameOptions};
//! use tower_helmet::HelmetLayer;
//!
//! // default layer with all security headers active
//! let layer = HelmetLayer::default();
//!
//! // default layer with customizations applied
//! let mut directives = HashMap::new();
//! directives.insert("default-src", vec!["'self'", "https://example.com"]);
//! directives.insert("img-src", vec!["'self'", "data:", "https://example.com"]);
//! directives.insert("script-src", vec!["'self'", "'unsafe-inline'", "https://example.com"]);
//! let csp = ContentSecurityPolicy {
//!   directives,
//!   ..Default::default()
//! };
//!
//! let layer = HelmetLayer::default()
//!     .disable_strict_transport_security()
//!     .disable_cross_origin_embedder_policy()
//!     .content_security_policy(csp);
//!
//! // completely blank layer, selectively enable and add headers
//! let layer = HelmetLayer::new()
//!   .x_frame_options(XFrameOptions::SameOrigin)
//!   .expect_ct(ExpectCt::default());
//! ```
pub mod header;

use futures::ready;
use http::header::{HeaderName, InvalidHeaderValue};
use http::{HeaderMap, HeaderValue, Request, Response};
use pin_project_lite::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower_layer::Layer;
use tower_service::Service;

use header::{
    ContentSecurityPolicy, CrossOriginEmbedderPolicy, CrossOriginOpenerPolicy,
    CrossOriginResourcePolicy, ExpectCt, OriginAgentCluster, ReferrerPolicy,
    StrictTransportSecurity, XContentTypeOptions, XDnsPrefetchControl, XDownloadOptions,
    XFrameOptions, XPermittedCrossDomainPolicies, XXSSProtection,
};

trait IntoHeader {
    fn header_name(&self) -> HeaderName;
    fn header_value(&self) -> Result<HeaderValue, InvalidHeaderValue>;
}

/// HelmetLayer
pub struct HelmetLayer<'a> {
    content_security_policy: Option<ContentSecurityPolicy<'a>>,
    cross_origin_embedder_policy: Option<CrossOriginEmbedderPolicy>,
    cross_origin_opener_policy: Option<CrossOriginOpenerPolicy>,
    cross_origin_resource_policy: Option<CrossOriginResourcePolicy>,
    expect_ct: Option<ExpectCt>,
    origin_agent_cluster: Option<OriginAgentCluster>,
    referrer_policy: Option<ReferrerPolicy>,
    strict_transport_security: Option<StrictTransportSecurity>,
    x_content_type_options: Option<XContentTypeOptions>,
    x_dns_prefetch_control: Option<XDnsPrefetchControl>,
    x_download_options: Option<XDownloadOptions>,
    x_frame_options: Option<XFrameOptions>,
    x_permitted_cross_domain_policies: Option<XPermittedCrossDomainPolicies>,
    x_xss_protection: Option<XXSSProtection>,
}

impl<'a> Default for HelmetLayer<'a> {
    fn default() -> Self {
        HelmetLayer {
            content_security_policy: Some(ContentSecurityPolicy::default()),
            cross_origin_embedder_policy: Some(CrossOriginEmbedderPolicy::default()),
            cross_origin_opener_policy: Some(CrossOriginOpenerPolicy::default()),
            cross_origin_resource_policy: Some(CrossOriginResourcePolicy::default()),
            expect_ct: Some(ExpectCt::default()),
            origin_agent_cluster: Some(OriginAgentCluster::default()),
            referrer_policy: Some(ReferrerPolicy::default()),
            strict_transport_security: Some(StrictTransportSecurity::default()),
            x_content_type_options: Some(XContentTypeOptions::default()),
            x_dns_prefetch_control: Some(XDnsPrefetchControl::default()),
            x_download_options: Some(XDownloadOptions::default()),
            x_frame_options: Some(XFrameOptions::default()),
            x_permitted_cross_domain_policies: Some(XPermittedCrossDomainPolicies::default()),
            x_xss_protection: Some(XXSSProtection::default()),
        }
    }
}

impl<'a> HelmetLayer<'a> {
    pub fn new() -> Self {
        HelmetLayer {
            content_security_policy: None,
            cross_origin_embedder_policy: None,
            cross_origin_opener_policy: None,
            cross_origin_resource_policy: None,
            expect_ct: None,
            origin_agent_cluster: None,
            referrer_policy: None,
            strict_transport_security: None,
            x_content_type_options: None,
            x_dns_prefetch_control: None,
            x_download_options: None,
            x_frame_options: None,
            x_permitted_cross_domain_policies: None,
            x_xss_protection: None,
        }
    }

    pub fn recommended_defaults() -> Self {
        HelmetLayer::default()
    }

    pub fn content_security_policy(&mut self, v: ContentSecurityPolicy<'a>) -> &mut Self {
        self.content_security_policy = Some(v);
        self
    }
    pub fn disable_content_security_policy(&mut self) -> &mut Self {
        self.content_security_policy = None;
        self
    }

    pub fn cross_origin_embedder_policy(&mut self, v: CrossOriginEmbedderPolicy) -> &mut Self {
        self.cross_origin_embedder_policy = Some(v);
        self
    }
    pub fn disable_cross_origin_embedder_policy(&mut self) -> &mut Self {
        self.cross_origin_embedder_policy = None;
        self
    }

    pub fn cross_origin_opener_policy(&mut self, v: CrossOriginOpenerPolicy) -> &mut Self {
        self.cross_origin_opener_policy = Some(v);
        self
    }
    pub fn disable_cross_origin_opener_policy(&mut self) -> &mut Self {
        self.cross_origin_opener_policy = None;
        self
    }

    pub fn cross_origin_resource_policy(&mut self, v: CrossOriginResourcePolicy) -> &mut Self {
        self.cross_origin_resource_policy = Some(v);
        self
    }
    pub fn disable_cross_origin_resource_policy(&mut self) -> &mut Self {
        self.cross_origin_resource_policy = None;
        self
    }

    pub fn expect_ct(&mut self, v: ExpectCt) -> &mut Self {
        self.expect_ct = Some(v);
        self
    }
    pub fn disable_expect_ct(&mut self) -> &mut Self {
        self.expect_ct = None;
        self
    }

    pub fn origin_agent_cluster(&mut self, v: OriginAgentCluster) -> &mut Self {
        self.origin_agent_cluster = Some(v);
        self
    }
    pub fn disable_origin_agent_cluster(&mut self) -> &mut Self {
        self.origin_agent_cluster = None;
        self
    }

    pub fn referrer_policy(&mut self, v: ReferrerPolicy) -> &mut Self {
        self.referrer_policy = Some(v);
        self
    }
    pub fn disable_referrer_policy(&mut self) -> &mut Self {
        self.referrer_policy = None;
        self
    }

    pub fn strict_transport_security(&mut self, v: StrictTransportSecurity) -> &mut Self {
        self.strict_transport_security = Some(v);
        self
    }
    pub fn disable_strict_transport_security(&mut self) -> &mut Self {
        self.strict_transport_security = None;
        self
    }

    pub fn x_content_type_options(&mut self, v: XContentTypeOptions) -> &mut Self {
        self.x_content_type_options = Some(v);
        self
    }
    pub fn disable_x_content_type_options(&mut self) -> &mut Self {
        self.x_content_type_options = None;
        self
    }

    pub fn x_dns_prefetch_control(&mut self, v: XDnsPrefetchControl) -> &mut Self {
        self.x_dns_prefetch_control = Some(v);
        self
    }
    pub fn disable_x_dns_prefetch_control(&mut self) -> &mut Self {
        self.x_dns_prefetch_control = None;
        self
    }

    pub fn x_download_options(&mut self, v: XDownloadOptions) -> &mut Self {
        self.x_download_options = Some(v);
        self
    }
    pub fn disable_x_download_options(&mut self) -> &mut Self {
        self.x_download_options = None;
        self
    }

    pub fn x_frame_options(&mut self, v: XFrameOptions) -> &mut Self {
        self.x_frame_options = Some(v);
        self
    }
    pub fn disable_x_frame_options(&mut self) -> &mut Self {
        self.x_frame_options = None;
        self
    }

    pub fn x_permitted_cross_domain_policies(
        &mut self,
        v: XPermittedCrossDomainPolicies,
    ) -> &mut Self {
        self.x_permitted_cross_domain_policies = Some(v);
        self
    }
    pub fn disable_x_permitted_cross_domain_policies(&mut self) -> &mut Self {
        self.x_permitted_cross_domain_policies = None;
        self
    }

    pub fn x_xss_protection(&mut self, v: XXSSProtection) -> &mut Self {
        self.x_xss_protection = Some(v);
        self
    }
    pub fn disable_x_xss_protection(&mut self) -> &mut Self {
        self.x_xss_protection = None;
        self
    }
}

impl<'a, S> Layer<S> for HelmetLayer<'a> {
    type Service = HelmetService<S>;

    fn layer(&self, service: S) -> Self::Service {
        let mut headers = HeaderMap::new();

        if let Some(h) = &self.content_security_policy {
            headers.insert(h.header_name(), h.header_value().unwrap());
        }
        if let Some(h) = &self.cross_origin_embedder_policy {
            headers.insert(h.header_name(), h.header_value().unwrap());
        }
        if let Some(h) = &self.cross_origin_opener_policy {
            headers.insert(h.header_name(), h.header_value().unwrap());
        }
        if let Some(h) = &self.cross_origin_resource_policy {
            headers.insert(h.header_name(), h.header_value().unwrap());
        }
        if let Some(h) = &self.expect_ct {
            headers.insert(h.header_name(), h.header_value().unwrap());
        }
        if let Some(h) = &self.origin_agent_cluster {
            headers.insert(h.header_name(), h.header_value().unwrap());
        }
        if let Some(h) = &self.referrer_policy {
            headers.insert(h.header_name(), h.header_value().unwrap());
        }
        if let Some(h) = &self.strict_transport_security {
            headers.insert(h.header_name(), h.header_value().unwrap());
        }
        if let Some(h) = &self.x_content_type_options {
            headers.insert(h.header_name(), h.header_value().unwrap());
        }
        if let Some(h) = &self.x_dns_prefetch_control {
            headers.insert(h.header_name(), h.header_value().unwrap());
        }
        if let Some(h) = &self.x_download_options {
            headers.insert(h.header_name(), h.header_value().unwrap());
        }
        if let Some(h) = &self.x_frame_options {
            headers.insert(h.header_name(), h.header_value().unwrap());
        }
        if let Some(h) = &self.x_permitted_cross_domain_policies {
            headers.insert(h.header_name(), h.header_value().unwrap());
        }
        if let Some(h) = &self.x_xss_protection {
            headers.insert(h.header_name(), h.header_value().unwrap());
        }

        HelmetService {
            inner: service,
            headers,
        }
    }
}

#[derive(Clone)]
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

        for (name, value) in this.headers.iter() {
            headers.insert(name, value.clone());
        }

        Poll::Ready(Ok(res))
    }
}
