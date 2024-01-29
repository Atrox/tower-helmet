use std::fmt::{Display, Formatter};

use http::header::{HeaderName, InvalidHeaderValue};
use http::HeaderValue;

use crate::IntoHeader;

/// `CrossOriginOpenerPolicy` sets the `Cross-Origin-Opener-Policy` header.
/// For more, see [MDN's article on this header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Opener-Policy).
#[derive(Debug, Clone, Copy, Default)]
pub enum CrossOriginOpenerPolicy {
    UnsafeNone,
    SameOriginAllowPopups,
    #[default]
    SameOrigin,
}

impl Display for CrossOriginOpenerPolicy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CrossOriginOpenerPolicy::UnsafeNone => "unsafe-none",
            CrossOriginOpenerPolicy::SameOriginAllowPopups => "same-origin-allow-popups",
            CrossOriginOpenerPolicy::SameOrigin => "same-origin",
        };

        write!(f, "{}", s)
    }
}

impl IntoHeader for CrossOriginOpenerPolicy {
    fn header_name(&self) -> HeaderName {
        HeaderName::from_static("cross-origin-opener-policy")
    }

    fn header_value(&self) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(self.to_string().as_str())
    }
}
