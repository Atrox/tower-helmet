use crate::IntoHeader;
use http::header::{HeaderName, InvalidHeaderValue};
use http::HeaderValue;
use std::fmt::{Display, Formatter};

/// `XFrameOptions` sets the `X-Frame-Options` header to help you mitigate [clickjacking attacks](https://en.wikipedia.org/wiki/Clickjacking).
/// This header is superseded by [the `frame-ancestors` Content Security Policy directive](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Security-Policy/frame-ancestors) but is still useful on old browsers.
/// For more, see `helmet.contentSecurityPolicy`, as well as [the documentation on MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Frame-Options).
///
/// `DENY` or `SAMEORIGIN`. (A legacy directive, `ALLOW-FROM`, is not supported by this crate. [Read more here.](https://github.com/helmetjs/helmet/wiki/How-to-use-X%E2%80%93Frame%E2%80%93Options's-%60ALLOW%E2%80%93FROM%60-directive))
pub enum XFrameOptions {
    Deny,
    SameOrigin,
}

impl Display for XFrameOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            XFrameOptions::SameOrigin => "SAMEORIGIN",
            XFrameOptions::Deny => "DENY",
        };

        write!(f, "{}", s)
    }
}

impl Default for XFrameOptions {
    fn default() -> Self {
        XFrameOptions::SameOrigin
    }
}

impl IntoHeader for XFrameOptions {
    fn header_name(&self) -> HeaderName {
        http::header::X_FRAME_OPTIONS
    }

    fn header_value(&self) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(self.to_string().as_str())
    }
}
