# tower-helmet

[![Crates.io](https://img.shields.io/crates/v/tower-helmet)](https://crates.io/crates/tower-helmet)
[![Documentation](https://docs.rs/tower-helmet/badge.svg)](https://docs.rs/tower-helmet)
[![License](https://img.shields.io/crates/l/tower-helmet)](LICENSE)

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
let layer = HelmetLayer::with_defaults();

// default layer with customizations applied
let mut directives = HashMap::new();
directives.insert("default-src", vec!["'self'", "https://example.com"]);
directives.insert("img-src", vec!["'self'", "data:", "https://example.com"]);
directives.insert("script-src", vec!["'self'", "'unsafe-inline'", "https://example.com"]);
let csp = ContentSecurityPolicy {
  directives,
  ..Default::default()
};

let layer = HelmetLayer::with_defaults().enable(csp);

// completely blank layer, selectively enable and add headers
let layer = HelmetLayer::blank()
  .enable(XFrameOptions::SameOrigin)
  .enable(ExpectCt::default());
```
