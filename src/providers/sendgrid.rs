//! SendGrid Event Webhook.

use crate::common::{Webhook, WebhookBuilder};
use crate::error::Result;
use crate::provider::Provider;

provider_events! {
    /// Common SendGrid Event Webhook event types.
    pub enum Event {
        /// `processed`
        Processed => "processed",
        /// `dropped`
        Dropped => "dropped",
        /// `delivered`
        Delivered => "delivered",
        /// `deferred`
        Deferred => "deferred",
        /// `bounce`
        Bounce => "bounce",
        /// `blocked`
        Blocked => "blocked",
        /// `open`
        Open => "open",
        /// `click`
        Click => "click",
        /// `spamreport`
        SpamReport => "spamreport",
        /// `unsubscribe`
        Unsubscribe => "unsubscribe",
        /// `group_unsubscribe`
        GroupUnsubscribe => "group_unsubscribe",
        /// `group_resubscribe`
        GroupResubscribe => "group_resubscribe",
    }
}

/// Start a SendGrid Event Webhook builder.
#[must_use]
pub fn endpoint() -> WebhookBuilder {
    WebhookBuilder::new(Provider::SendGrid)
}

/// Build a SendGrid webhook from typed events.
pub fn webhook(
    url: impl Into<String>,
    events: impl IntoIterator<Item = Event>,
) -> Result<Webhook> {
    endpoint().url(url).events(events).build()
}
