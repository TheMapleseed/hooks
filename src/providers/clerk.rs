//! Clerk (Svix) webhooks.

use crate::common::{Webhook, WebhookBuilder};
use crate::error::Result;
use crate::provider::Provider;

provider_events! {
    /// Common Clerk webhook event types.
    pub enum Event {
        /// `user.created`
        UserCreated => "user.created",
        /// `user.updated`
        UserUpdated => "user.updated",
        /// `user.deleted`
        UserDeleted => "user.deleted",
        /// `session.created`
        SessionCreated => "session.created",
        /// `session.ended`
        SessionEnded => "session.ended",
        /// `session.removed`
        SessionRemoved => "session.removed",
        /// `session.revoked`
        SessionRevoked => "session.revoked",
        /// `email.created`
        EmailCreated => "email.created",
        /// `sms.created`
        SmsCreated => "sms.created",
        /// `organization.created`
        OrganizationCreated => "organization.created",
        /// `organization.updated`
        OrganizationUpdated => "organization.updated",
        /// `organization.deleted`
        OrganizationDeleted => "organization.deleted",
        /// `organizationMembership.created`
        OrganizationMembershipCreated => "organizationMembership.created",
        /// `organizationMembership.updated`
        OrganizationMembershipUpdated => "organizationMembership.updated",
        /// `organizationMembership.deleted`
        OrganizationMembershipDeleted => "organizationMembership.deleted",
        /// `organizationInvitation.created`
        OrganizationInvitationCreated => "organizationInvitation.created",
        /// `organizationInvitation.accepted`
        OrganizationInvitationAccepted => "organizationInvitation.accepted",
        /// `organizationInvitation.revoked`
        OrganizationInvitationRevoked => "organizationInvitation.revoked",
    }
}

/// Start a Clerk webhook endpoint builder.
#[must_use]
pub fn endpoint() -> WebhookBuilder {
    WebhookBuilder::new(Provider::Clerk)
}

/// Build a Clerk webhook from typed events.
pub fn webhook(
    url: impl Into<String>,
    events: impl IntoIterator<Item = Event>,
) -> Result<Webhook> {
    endpoint().url(url).events(events).build()
}
