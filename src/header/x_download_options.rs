use http::header::{HeaderName, InvalidHeaderValue};
use http::HeaderValue;

use crate::IntoHeader;

/// `XDownloadOptions` sets the `X-Download-Options` header, which is specific to Internet Explorer
/// 8. It forces potentially-unsafe downloads to be saved, mitigating execution of HTML in your
/// site's context. For more, see [this old post on MSDN](https://docs.microsoft.com/en-us/archive/blogs/ie/ie8-security-part-v-comprehensive-protection).
pub struct XDownloadOptions;

impl Default for XDownloadOptions {
    fn default() -> Self {
        XDownloadOptions
    }
}

impl IntoHeader for XDownloadOptions {
    fn header_name(&self) -> HeaderName {
        HeaderName::from_static("x-download-options")
    }

    fn header_value(&self) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str("noopen")
    }
}
