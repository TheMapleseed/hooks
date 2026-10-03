//! High-level Swiss-army-knife entry points: describe → register → deliver.

use crate::common::Webhook;
use crate::deliver::{Delivery, DeliveryClient, DeliveryResponse};
use crate::error::Result;
use crate::register::{GitHubRegistrar, RegisteredEndpoint, Registrar, StripeRegistrar};
use crate::secret::Secret;

/// Unified client for outbound delivery and provider registration.
#[derive(Debug, Clone, Default)]
pub struct Client {
    delivery: DeliveryClient,
}

impl Client {
    /// Create a client with default HTTP settings.
    #[must_use]
    pub fn new() -> Self {
        Self {
            delivery: DeliveryClient::new(),
        }
    }

    /// Sign and POST `delivery` using `secret`.
    pub fn send(&self, delivery: &Delivery, secret: &Secret) -> Result<DeliveryResponse> {
        self.delivery.send(delivery, secret)
    }

    /// Deliver `body` for `event` to a configured [`Webhook`] endpoint.
    pub fn send_to_webhook(
        &self,
        webhook: &Webhook,
        event: impl AsRef<str>,
        body: impl AsRef<[u8]>,
        secret: &Secret,
    ) -> Result<DeliveryResponse> {
        let delivery = Delivery::to_webhook(webhook, event, body);
        self.send(&delivery, secret)
    }

    /// Register a Stripe webhook endpoint via the Stripe API.
    pub fn register_stripe(
        &self,
        api_key: Secret,
        webhook: &Webhook,
    ) -> Result<RegisteredEndpoint> {
        StripeRegistrar::new(api_key).register(webhook)
    }

    /// Register a GitHub repository webhook via the GitHub API.
    pub fn register_github(
        &self,
        token: Secret,
        owner: impl Into<String>,
        repo: impl Into<String>,
        webhook: &Webhook,
    ) -> Result<RegisteredEndpoint> {
        GitHubRegistrar::new(token, owner, repo).register(webhook)
    }
}
