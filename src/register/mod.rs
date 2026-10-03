//! Register / create webhook endpoints with provider APIs.

mod github;
mod stripe;

use crate::common::Webhook;
use crate::error::Result;

pub use github::{GitHubRegistration, GitHubRegistrar};
pub use stripe::{StripeRegistration, StripeRegistrar};

/// Result of registering a webhook with a remote provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisteredEndpoint {
    /// Provider id.
    pub provider: &'static str,
    /// Provider-assigned id when available (`we_…`, hook id, …).
    pub id: Option<String>,
    /// Delivery URL that was registered.
    pub url: String,
    /// Events/topics that were registered.
    pub events: Vec<String>,
    /// Signing secret returned by the provider when available (Stripe).
    ///
    /// Callers should store this in a secret manager immediately; it is not
    /// written to disk by this crate.
    pub signing_secret: Option<String>,
    /// Extra provider response notes (status, path, …).
    pub details: Vec<(String, String)>,
}

/// Common registration surface implemented by provider registrars.
pub trait Registrar {
    /// Create or update the remote endpoint described by `webhook`.
    fn register(&self, webhook: &Webhook) -> Result<RegisteredEndpoint>;
}
