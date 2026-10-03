//! Linear issue webhooks.

use crate::common::{Webhook, WebhookBuilder};
use crate::error::Result;
use crate::provider::Provider;

provider_events! {
    /// Common Linear webhook event types.
    pub enum Event {
        /// `Issue`
        Issue => "Issue",
        /// `Comment`
        Comment => "Comment",
        /// `IssueLabel`
        IssueLabel => "IssueLabel",
        /// `Project`
        Project => "Project",
        /// `Cycle`
        Cycle => "Cycle",
        /// `Reaction`
        Reaction => "Reaction",
        /// `Attachment`
        Attachment => "Attachment",
        /// `Customer`
        Customer => "Customer",
        /// `CustomerNeed`
        CustomerNeed => "CustomerNeed",
        /// `Document`
        Document => "Document",
        /// `Initiative`
        Initiative => "Initiative",
    }
}

/// Start a Linear webhook endpoint builder.
#[must_use]
pub fn endpoint() -> WebhookBuilder {
    WebhookBuilder::new(Provider::Linear)
}

/// Build a Linear webhook from typed events.
pub fn webhook(
    url: impl Into<String>,
    events: impl IntoIterator<Item = Event>,
) -> Result<Webhook> {
    endpoint().url(url).events(events).build()
}
