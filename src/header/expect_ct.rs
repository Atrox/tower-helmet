use crate::IntoHeader;
use http::header::{HeaderName, InvalidHeaderValue};
use http::HeaderValue;
use std::time::Duration;

/// `ExpectCt` sets the `Expect-CT` header which helps mitigate misissued SSL certificates.
/// See [MDN's article on Certificate Transparency](https://developer.mozilla.org/en-US/docs/Web/Security/Certificate_Transparency) and the [`Expect-CT` header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Expect-CT) for more.
pub struct ExpectCt {
    /// `max_age` is the number of seconds to expect Certificate Transparency.
    pub max_age: Duration,
    /// If `true`, the user agent (usually a browser) should refuse future connections that violate its Certificate Transparency policy.
    pub enforce: bool,
    /// If set, complying user agents will report Certificate Transparency failures to this URL.
    pub report_uri: Option<String>,
}

impl Default for ExpectCt {
    fn default() -> Self {
        ExpectCt {
            max_age: Duration::from_secs(0),
            enforce: false,
            report_uri: None,
        }
    }
}

impl IntoHeader for ExpectCt {
    fn header_name(&self) -> HeaderName {
        HeaderName::from_static("expect-ct")
    }

    fn header_value(&self) -> Result<HeaderValue, InvalidHeaderValue> {
        let mut directives = vec![format!("max-age={}", self.max_age.as_secs())];

        if self.enforce {
            directives.push("enforce".to_owned());
        }

        if let Some(report_uri) = self.report_uri.as_ref() {
            directives.push(format!("report-uri={}", report_uri));
        }

        HeaderValue::from_str(directives.join(", ").as_str())
    }
}
