use http::header::{HeaderName, InvalidHeaderValue};
use http::HeaderValue;

use crate::IntoHeader;

/// `OriginAgentCluster` sets the `Origin-Agent-Cluster` header, which provides a mechanism to allow
/// web applications to isolate their origins. Read more about it [in the spec](https://whatpr.org/html/6214/origin.html#origin-keyed-agent-clusters).
pub struct OriginAgentCluster;

impl Default for OriginAgentCluster {
    fn default() -> Self {
        OriginAgentCluster
    }
}

impl IntoHeader for OriginAgentCluster {
    fn header_name(&self) -> HeaderName {
        HeaderName::from_static("origin-agent-cluster")
    }

    fn header_value(&self) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str("?1")
    }
}
