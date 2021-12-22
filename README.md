# tower-helmet

this is still very **work in progress**

a port of the beautiful [helmet.js](https://github.com/helmetjs/helmet) in the javascript world.

`tower-helmet` helps you secure your tower server by setting various HTTP headers. _It's not a silver bullet_, but it can help!

You can find a list of all available headers under the [header] module. By default (with [HelmetLayer::default]) **all of them** are enabled.
Please take a good look at [ContentSecurityPolicy]. Most of the time you will need to adapt this one to your needs.

# Examples

```rust
use tower_helmet::header::{ContentSecurityPolicy, ExpectCt, XFrameOptions};
use tower_helmet::HelmetLayer;

// default layer with all security headers active
let layer = HelmetLayer::default();

// default layer with customizations applied
let mut directives = HashMap::new();
directives.insert("default-src", vec!["'self'", "https://example.com"]);
directives.insert("img-src", vec!["'self'", "data:", "https://example.com"]);
directives.insert("script-src", vec!["'self'", "'unsafe-inline'", "https://example.com"]);
let csp = ContentSecurityPolicy {
  directives,
  ..Default::default()
};

let layer = HelmetLayer::default()
    .disable_strict_transport_security()
    .disable_cross_origin_embedder_policy()
    .content_security_policy(csp);

// completely blank layer, selectively enable and add headers
let layer = HelmetLayer::new()
  .x_frame_options(XFrameOptions::SameOrigin)
  .expect_ct(ExpectCt::default());
```
