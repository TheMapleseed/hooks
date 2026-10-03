//! HubSpot CRM webhooks.

use crate::common::{Webhook, WebhookBuilder};
use crate::error::Result;
use crate::provider::Provider;

provider_events! {
    /// Common HubSpot webhook subscription types.
    pub enum Event {
        /// `contact.creation`
        ContactCreation => "contact.creation",
        /// `contact.deletion`
        ContactDeletion => "contact.deletion",
        /// `contact.propertyChange`
        ContactPropertyChange => "contact.propertyChange",
        /// `company.creation`
        CompanyCreation => "company.creation",
        /// `company.deletion`
        CompanyDeletion => "company.deletion",
        /// `company.propertyChange`
        CompanyPropertyChange => "company.propertyChange",
        /// `deal.creation`
        DealCreation => "deal.creation",
        /// `deal.deletion`
        DealDeletion => "deal.deletion",
        /// `deal.propertyChange`
        DealPropertyChange => "deal.propertyChange",
        /// `ticket.creation`
        TicketCreation => "ticket.creation",
        /// `ticket.deletion`
        TicketDeletion => "ticket.deletion",
        /// `ticket.propertyChange`
        TicketPropertyChange => "ticket.propertyChange",
        /// `conversation.creation`
        ConversationCreation => "conversation.creation",
        /// `conversation.newMessage`
        ConversationNewMessage => "conversation.newMessage",
    }
}

/// Start a HubSpot webhook endpoint builder.
#[must_use]
pub fn endpoint() -> WebhookBuilder {
    WebhookBuilder::new(Provider::HubSpot)
}

/// Build a HubSpot webhook from typed events.
pub fn webhook(
    url: impl Into<String>,
    events: impl IntoIterator<Item = Event>,
) -> Result<Webhook> {
    endpoint().url(url).events(events).build()
}
