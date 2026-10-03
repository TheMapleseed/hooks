//! Vercel deployment / log drain hooks.

use crate::common::{Webhook, WebhookBuilder};
use crate::error::Result;
use crate::provider::Provider;

provider_events! {
    /// Common Vercel webhook event types.
    pub enum Event {
        /// `deployment.created`
        DeploymentCreated => "deployment.created",
        /// `deployment.succeeded`
        DeploymentSucceeded => "deployment.succeeded",
        /// `deployment.error`
        DeploymentError => "deployment.error",
        /// `deployment.canceled`
        DeploymentCanceled => "deployment.canceled",
        /// `deployment.promoted`
        DeploymentPromoted => "deployment.promoted",
        /// `deployment.cleanup`
        DeploymentCleanup => "deployment.cleanup",
        /// `project.created`
        ProjectCreated => "project.created",
        /// `project.removed`
        ProjectRemoved => "project.removed",
        /// `domain.created`
        DomainCreated => "domain.created",
        /// `integration-configuration.permission-upgraded`
        IntegrationPermissionUpgraded => "integration-configuration.permission-upgraded",
        /// `integration-configuration.removed`
        IntegrationRemoved => "integration-configuration.removed",
        /// `integration-configuration.scope-change-confirmed`
        IntegrationScopeChangeConfirmed => "integration-configuration.scope-change-confirmed",
        /// Log drain payload.
        LogDrain => "log.drain",
    }
}

/// Start a Vercel webhook endpoint builder.
#[must_use]
pub fn endpoint() -> WebhookBuilder {
    WebhookBuilder::new(Provider::Vercel)
}

/// Build a Vercel webhook from typed events.
pub fn webhook(
    url: impl Into<String>,
    events: impl IntoIterator<Item = Event>,
) -> Result<Webhook> {
    endpoint().url(url).events(events).build()
}
