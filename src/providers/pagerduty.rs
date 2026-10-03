//! PagerDuty webhooks.

use crate::common::{Webhook, WebhookBuilder};
use crate::error::Result;
use crate::provider::Provider;

provider_events! {
    /// Common PagerDuty webhook / event subscription types.
    pub enum Event {
        /// `incident.triggered`
        IncidentTriggered => "incident.triggered",
        /// `incident.acknowledged`
        IncidentAcknowledged => "incident.acknowledged",
        /// `incident.resolved`
        IncidentResolved => "incident.resolved",
        /// `incident.escalated`
        IncidentEscalated => "incident.escalated",
        /// `incident.reassigned`
        IncidentReassigned => "incident.reassigned",
        /// `incident.annotated`
        IncidentAnnotated => "incident.annotated",
        /// `incident.unacknowledged`
        IncidentUnacknowledged => "incident.unacknowledged",
        /// `incident.delegated`
        IncidentDelegated => "incident.delegated",
        /// `incident.priority_updated`
        IncidentPriorityUpdated => "incident.priority_updated",
        /// `service.created`
        ServiceCreated => "service.created",
        /// `service.updated`
        ServiceUpdated => "service.updated",
    }
}

/// Start a PagerDuty webhook endpoint builder.
#[must_use]
pub fn endpoint() -> WebhookBuilder {
    WebhookBuilder::new(Provider::PagerDuty)
}

/// Build a PagerDuty webhook from typed events.
pub fn webhook(
    url: impl Into<String>,
    events: impl IntoIterator<Item = Event>,
) -> Result<Webhook> {
    endpoint().url(url).events(events).build()
}
