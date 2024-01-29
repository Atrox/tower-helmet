use std::collections::HashMap;

use http::header::{HeaderName, InvalidHeaderValue};
use http::HeaderValue;
use lazy_static::lazy_static;

use crate::IntoHeader;

lazy_static! {
    static ref DEFAULT_DIRECTIVES: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("default-src", vec!["'self'"]);
        m.insert("base-uri", vec!["'self'"]);
        m.insert("block-all-mixed-content", vec![]);
        m.insert("font-src", vec!["'self'", "https:", "data:"]);
        m.insert("frame-ancestors", vec!["'self'"]);
        m.insert("img-src", vec!["'self'", "data:"]);
        m.insert("object-src", vec!["'none'"]);
        m.insert("script-src", vec!["'self'"]);
        m.insert("script-src-attr", vec!["'none'"]);
        m.insert("style-src", vec!["'self'", "https:", "'unsafe-inline'"]);
        m.insert("upgrade-insecure-requests", vec![]);
        m
    };
}

/// `ContentSecurityPolicy` sets the `Content-Security-Policy` header which helps mitigate
/// cross-site scripting attacks, among other things. See [MDN's introductory article on Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP).
///
/// This middleware performs very little validation. You should rely on CSP checkers like [CSP Evaluator](https://csp-evaluator.withgoogle.com/) instead.
///
/// If no directive is supplied and `use_defaults` is `true`, the following policy is set
/// (whitespace added for readability): ```text
/// default-src 'self';
/// base-uri 'self';
/// block-all-mixed-content;
/// font-src 'self' https: data:;
/// frame-ancestors 'self';
/// img-src 'self' data:;
/// object-src 'none';
/// script-src 'self';
/// script-src-attr 'none';
/// style-src 'self' https: 'unsafe-inline';
/// upgrade-insecure-requests
/// ```
#[derive(Debug, Clone)]
pub struct ContentSecurityPolicy<'a> {
    pub use_defaults: bool,
    /// Each key is the directive name in kebab case (such as `default-src`).
    /// Each value is a vector of strings for that directive
    pub directives: HashMap<&'a str, Vec<&'a str>>,
    /// If `true`, [the `Content-Security-Policy-Report-Only` header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Security-Policy-Report-Only) will be set instead.
    pub report_only: bool,
}

impl ContentSecurityPolicy<'static> {
    /// Returns the default directives
    ///
    /// ```text
    /// default-src 'self';
    /// base-uri 'self';
    /// block-all-mixed-content;
    /// font-src 'self' https: data:;
    /// frame-ancestors 'self';
    /// img-src 'self' data:;
    /// object-src 'none';
    /// script-src 'self';
    /// script-src-attr 'none';
    /// style-src 'self' https: 'unsafe-inline';
    /// upgrade-insecure-requests
    /// ```
    pub fn default_directives() -> &'static HashMap<&'static str, Vec<&'static str>> {
        &DEFAULT_DIRECTIVES
    }
}

impl<'a> Default for ContentSecurityPolicy<'a> {
    fn default() -> Self {
        ContentSecurityPolicy {
            use_defaults: true,
            directives: HashMap::new(),
            report_only: false,
        }
    }
}

impl<'a> IntoHeader for ContentSecurityPolicy<'a> {
    fn header_name(&self) -> HeaderName {
        if self.report_only {
            http::header::CONTENT_SECURITY_POLICY_REPORT_ONLY
        } else {
            http::header::CONTENT_SECURITY_POLICY
        }
    }

    fn header_value(&self) -> Result<HeaderValue, InvalidHeaderValue> {
        let directives = if self.use_defaults {
            if self.directives.is_empty() {
                DEFAULT_DIRECTIVES.clone()
            } else {
                let mut directives = DEFAULT_DIRECTIVES.clone();
                directives.extend(self.directives.clone());

                directives
            }
        } else {
            self.directives.clone()
        };

        let header = directives
            .iter()
            .map(|(key, values)| format!("{} {}", key, values.join(" ")))
            .collect::<Vec<String>>()
            .join("; ");

        HeaderValue::from_str(header.trim())
    }
}
