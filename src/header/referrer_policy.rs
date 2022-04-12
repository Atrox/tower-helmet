use std::fmt::{Display, Formatter};

use http::header::{HeaderName, InvalidHeaderValue};
use http::HeaderValue;

use crate::IntoHeader;

/// `ReferrerPolicy` sets the `Referrer-Policy` header which controls what information is set in [the `Referer` header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referer).
/// See ["Referer header: privacy and security concerns"](https://developer.mozilla.org/en-US/docs/Web/Security/Referer_header:_privacy_and_security_concerns) and [the header's documentation](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referrer-Policy) on MDN for more.
pub struct ReferrerPolicy(pub Vec<ReferrerPolicyValue>);

pub enum ReferrerPolicyValue {
    NoReferrer,
    NoReferrerWhenDowngrade,
    Origin,
    OriginWhenCrossOrigin,
    SameOrigin,
    StrictOrigin,
    StrictOriginWhenCrossOrigin,
}

impl Display for ReferrerPolicyValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ReferrerPolicyValue::NoReferrer => "no-referrer",
            ReferrerPolicyValue::NoReferrerWhenDowngrade => "no-referrer-when-downgrade",
            ReferrerPolicyValue::Origin => "origin",
            ReferrerPolicyValue::OriginWhenCrossOrigin => "origin-when-cross-origin",
            ReferrerPolicyValue::SameOrigin => "same-origin",
            ReferrerPolicyValue::StrictOrigin => "strict-origin",
            ReferrerPolicyValue::StrictOriginWhenCrossOrigin => "strict-origin-when-cross-origin",
        };

        write!(f, "{}", s)
    }
}

impl Default for ReferrerPolicy {
    fn default() -> Self {
        ReferrerPolicy(vec![ReferrerPolicyValue::NoReferrer])
    }
}

impl IntoHeader for ReferrerPolicy {
    fn header_name(&self) -> HeaderName {
        http::header::REFERRER_POLICY
    }

    fn header_value(&self) -> Result<HeaderValue, InvalidHeaderValue> {
        let s: Vec<String> = self.0.iter().map(|v| v.to_string()).collect();
        HeaderValue::from_str(s.join(",").as_str())
    }
}
