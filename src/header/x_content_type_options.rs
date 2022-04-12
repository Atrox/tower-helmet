use http::header::{HeaderName, InvalidHeaderValue};
use http::HeaderValue;

use crate::IntoHeader;

/// `XContentTypeOptions` sets the `X-Content-Type-Options` header to `nosniff`.
/// This mitigates [MIME type sniffing](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types#MIME_sniffing) which can cause security vulnerabilities.
/// See [documentation for this header on MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Content-Type-Options) for more.
pub struct XContentTypeOptions;

impl Default for XContentTypeOptions {
    fn default() -> Self {
        XContentTypeOptions
    }
}

impl IntoHeader for XContentTypeOptions {
    fn header_name(&self) -> HeaderName {
        http::header::X_CONTENT_TYPE_OPTIONS
    }

    fn header_value(&self) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str("nosniff")
    }
}
