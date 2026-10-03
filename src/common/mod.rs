//! Shared webhook endpoint model used by every HTTP provider.
//!
//! Providers only expose typed events and thin wrappers; the accessible surface
//! for "what is a webhook" lives here — URL, events, signature metadata — not
//! an HTTP client or template pack.

mod emit;
mod endpoint;
mod format;
mod signature;

pub use emit::webhook_to_kdl;
pub use endpoint::{Webhook, WebhookBuilder};
pub use format::EmitFormat;
pub use signature::{SignatureEncoding, SignatureScheme};
