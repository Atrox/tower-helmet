use crate::IntoHeader;
use http::header::{HeaderName, InvalidHeaderValue};
use http::HeaderValue;
use std::fmt::{Display, Formatter};

/// `CrossOriginResourcePolicy` sets the `Cross-Origin-Resource-Policy` header.
/// For more, see ["Consider deploying Cross-Origin Resource Policy](https://resourcepolicy.fyi/) and [MDN's article on this header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Resource-Policy).
pub enum CrossOriginResourcePolicy {
    SameSite,
    SameOrigin,
    CrossOrigin,
}

impl Display for CrossOriginResourcePolicy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CrossOriginResourcePolicy::SameSite => "same-site",
            CrossOriginResourcePolicy::SameOrigin => "same-origin",
            CrossOriginResourcePolicy::CrossOrigin => "cross-origin",
        };

        write!(f, "{}", s)
    }
}

impl Default for CrossOriginResourcePolicy {
    fn default() -> Self {
        CrossOriginResourcePolicy::SameOrigin
    }
}

impl IntoHeader for CrossOriginResourcePolicy {
    fn header_name(&self) -> HeaderName {
        HeaderName::from_static("cross-origin-resource-policy")
    }

    fn header_value(&self) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(self.to_string().as_str())
    }
}
