use std::fmt::{Display, Formatter};

use http::header::{HeaderName, InvalidHeaderValue};
use http::HeaderValue;

use crate::IntoHeader;

/// `XPermittedCrossDomainPolicies` sets the `X-Permitted-Cross-Domain-Policies` header, which tells
/// some clients (mostly Adobe products) your domain's policy for loading cross-domain content. See [the description on OWASP](https://owasp.org/www-project-secure-headers/) for more.
#[derive(Debug, Clone, Copy, Default)]
pub enum XPermittedCrossDomainPolicies {
    #[default]
    None,
    MasterOnly,
    ByContentType,
    All,
}

impl Display for XPermittedCrossDomainPolicies {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            XPermittedCrossDomainPolicies::None => "none",
            XPermittedCrossDomainPolicies::MasterOnly => "master-only",
            XPermittedCrossDomainPolicies::ByContentType => "by-content-type",
            XPermittedCrossDomainPolicies::All => "all",
        };

        write!(f, "{}", s)
    }
}

impl IntoHeader for XPermittedCrossDomainPolicies {
    fn header_name(&self) -> HeaderName {
        HeaderName::from_static("x-permitted-cross-domain-policies")
    }

    fn header_value(&self) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(self.to_string().as_str())
    }
}
