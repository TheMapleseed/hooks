//! [Standard Webhooks](https://www.standardwebhooks.com/) (Svix ecosystem).

use crate::common::{Webhook, WebhookBuilder};
use crate::error::Result;
use crate::provider::Provider;

provider_events! {
    /// Generic Standard Webhooks event names commonly used by Svix-backed APIs.
    pub enum Event {
        /// Generic message / notification delivered.
        MessageDelivered => "message.delivered",
        /// Generic message bounced / failed.
        MessageFailed => "message.failed",
        /// Generic message opened (email-style).
        MessageOpened => "message.opened",
        /// Generic message clicked.
        MessageClicked => "message.clicked",
        /// Resource created.
        ResourceCreated => "resource.created",
        /// Resource updated.
        ResourceUpdated => "resource.updated",
        /// Resource deleted.
        ResourceDeleted => "resource.deleted",
        /// Ping / health event.
        Ping => "ping",
    }
}

/// Start a Standard Webhooks endpoint builder.
#[must_use]
pub fn endpoint() -> WebhookBuilder {
    WebhookBuilder::new(Provider::StandardWebhooks)
}

/// Build a Standard Webhooks endpoint from typed events.
pub fn webhook(
    url: impl Into<String>,
    events: impl IntoIterator<Item = Event>,
) -> Result<Webhook> {
    endpoint().url(url).events(events).build()
}
