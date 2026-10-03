//! GitLab project/group webhooks.

use crate::common::{Webhook, WebhookBuilder};
use crate::error::Result;
use crate::provider::Provider;

provider_events! {
    /// Common GitLab webhook event names.
    pub enum Event {
        /// `Push Hook`
        Push => "Push Hook",
        /// `Tag Push Hook`
        TagPush => "Tag Push Hook",
        /// `Issue Hook`
        Issue => "Issue Hook",
        /// `Note Hook`
        Note => "Note Hook",
        /// `Merge Request Hook`
        MergeRequest => "Merge Request Hook",
        /// `Wiki Page Hook`
        WikiPage => "Wiki Page Hook",
        /// `Pipeline Hook`
        Pipeline => "Pipeline Hook",
        /// `Job Hook`
        Job => "Job Hook",
        /// `Deployment Hook`
        Deployment => "Deployment Hook",
        /// `Feature Flag Hook`
        FeatureFlag => "Feature Flag Hook",
        /// `Release Hook`
        Release => "Release Hook",
        /// `Emoji Hook`
        Emoji => "Emoji Hook",
        /// `Member Hook`
        Member => "Member Hook",
        /// `Subgroup Hook`
        Subgroup => "Subgroup Hook",
        /// `Project Hook`
        Project => "Project Hook",
    }
}

/// Start a GitLab webhook endpoint builder.
#[must_use]
pub fn endpoint() -> WebhookBuilder {
    WebhookBuilder::new(Provider::GitLab)
}

/// Build a GitLab webhook from typed events.
pub fn webhook(
    url: impl Into<String>,
    events: impl IntoIterator<Item = Event>,
) -> Result<Webhook> {
    endpoint().url(url).events(events).build()
}
