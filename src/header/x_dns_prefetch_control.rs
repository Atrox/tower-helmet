use crate::IntoHeader;
use http::header::{HeaderName, InvalidHeaderValue};
use http::HeaderValue;

/// `XDnsPrefetchControl` sets the `X-DNS-Prefetch-Control` header to help control DNS prefetching, which can improve user privacy at the expense of performance.
/// See [documentation on MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-DNS-Prefetch-Control) for more.
#[derive(Default)]
pub struct XDnsPrefetchControl(
    /// Is indictating whether to enable DNS prefetching.
    pub bool,
);

impl IntoHeader for XDnsPrefetchControl {
    fn header_name(&self) -> HeaderName {
        http::header::X_DNS_PREFETCH_CONTROL
    }

    fn header_value(&self) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(if self.0 { "on" } else { "off" })
    }
}
