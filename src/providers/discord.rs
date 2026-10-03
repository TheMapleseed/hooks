//! Discord interactions / application event subscriptions.

use crate::common::{Webhook, WebhookBuilder};
use crate::error::Result;
use crate::provider::Provider;

provider_events! {
    /// Common Discord interaction / application webhook-oriented events.
    pub enum Event {
        /// `APPLICATION_AUTHORIZED`
        ApplicationAuthorized => "APPLICATION_AUTHORIZED",
        /// `APPLICATION_DEAUTHORIZED`
        ApplicationDeauthorized => "APPLICATION_DEAUTHORIZED",
        /// `ENTITLEMENT_CREATE`
        EntitlementCreate => "ENTITLEMENT_CREATE",
        /// `ENTITLEMENT_UPDATE`
        EntitlementUpdate => "ENTITLEMENT_UPDATE",
        /// `ENTITLEMENT_DELETE`
        EntitlementDelete => "ENTITLEMENT_DELETE",
        /// Interaction ping (type 1).
        InteractionPing => "interaction.ping",
        /// Application command interaction.
        InteractionApplicationCommand => "interaction.application_command",
        /// Message component interaction.
        InteractionMessageComponent => "interaction.message_component",
        /// Modal submit interaction.
        InteractionModalSubmit => "interaction.modal_submit",
        /// Outbound channel webhook execute (bot posts).
        ChannelWebhookExecute => "channel.webhook.execute",
    }
}

/// Start a Discord webhook endpoint builder.
#[must_use]
pub fn endpoint() -> WebhookBuilder {
    WebhookBuilder::new(Provider::Discord)
}

/// Build a Discord endpoint from typed events.
pub fn webhook(
    url: impl Into<String>,
    events: impl IntoIterator<Item = Event>,
) -> Result<Webhook> {
    endpoint().url(url).events(events).build()
}
