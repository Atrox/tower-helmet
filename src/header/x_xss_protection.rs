use http::header::{HeaderName, InvalidHeaderValue};
use http::HeaderValue;

use crate::IntoHeader;

/// `XXSSProtection` disables browsers' buggy cross-site scripting filter by setting the
/// `X-XSS-Protection` header to `0`. See [discussion about disabling the header here](https://github.com/helmetjs/helmet/issues/230) and [documentation on MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-XSS-Protection).
pub struct XXSSProtection;

impl Default for XXSSProtection {
    fn default() -> Self {
        XXSSProtection
    }
}

impl IntoHeader for XXSSProtection {
    fn header_name(&self) -> HeaderName {
        http::header::X_XSS_PROTECTION
    }

    fn header_value(&self) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str("0")
    }
}
