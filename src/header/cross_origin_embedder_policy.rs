use crate::IntoHeader;
use http::header::{HeaderName, InvalidHeaderValue};
use http::HeaderValue;

/// `CrossOriginEmbedderPolicy` sets the `Cross-Origin-Embedder-Policy` header to `require-corp`.
/// See [MDN's article on this header](https://developer.cdn.mozilla.net/en-US/docs/Web/HTTP/Headers/Cross-Origin-Embedder-Policy) for more.
pub struct CrossOriginEmbedderPolicy;

impl Default for CrossOriginEmbedderPolicy {
    fn default() -> Self {
        CrossOriginEmbedderPolicy
    }
}

impl IntoHeader for CrossOriginEmbedderPolicy {
    fn header_name(&self) -> HeaderName {
        HeaderName::from_static("cross-origin-embedder-policy")
    }

    fn header_value(&self) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str("require-corp")
    }
}
