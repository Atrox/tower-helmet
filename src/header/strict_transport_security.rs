use std::time::Duration;

use http::header::{HeaderName, InvalidHeaderValue};
use http::HeaderValue;

use crate::IntoHeader;

/// `StrictTransportSecurity` sets the `Strict-Transport-Security` header which tells browsers to
/// prefer HTTPS over insecure HTTP. See [the documentation on MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Strict-Transport-Security) for more.
#[derive(Debug, Clone, Copy)]
pub struct StrictTransportSecurity {
    /// `max_age` is the number of seconds browsers should remember to prefer HTTPS. It defaults to
    /// `15552000`, which is 180 days.
    pub max_age: Duration,
    /// `include_subdomains` dictates whether to include the `includeSubDomains` directive, which
    /// makes this policy extend to subdomains. It defaults to `true`.
    pub include_subdomains: bool,
    /// If true, it adds the `preload` directive, expressing intent to add your HSTS policy to
    /// browsers. See [the "Preloading Strict Transport Security" section on MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Strict-Transport-Security#Preloading_Strict_Transport_Security) for more.
    pub preload: bool,
}

impl Default for StrictTransportSecurity {
    fn default() -> Self {
        StrictTransportSecurity {
            max_age: Duration::from_secs(15552000),
            include_subdomains: true,
            preload: false,
        }
    }
}

impl IntoHeader for StrictTransportSecurity {
    fn header_name(&self) -> HeaderName {
        http::header::STRICT_TRANSPORT_SECURITY
    }

    fn header_value(&self) -> Result<HeaderValue, InvalidHeaderValue> {
        let mut directives = vec![format!("max-age={}", self.max_age.as_secs())];

        if self.include_subdomains {
            directives.push("includeSubdomains".to_owned());
        }

        if self.preload {
            directives.push("preload".to_owned());
        }

        HeaderValue::from_str(directives.join("; ").as_str())
    }
}
