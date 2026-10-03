//! Bitbucket Cloud webhooks.

use crate::common::{Webhook, WebhookBuilder};
use crate::error::Result;
use crate::provider::Provider;

provider_events! {
    /// Common Bitbucket Cloud webhook events.
    pub enum Event {
        /// `repo:push`
        RepoPush => "repo:push",
        /// `repo:fork`
        RepoFork => "repo:fork",
        /// `repo:updated`
        RepoUpdated => "repo:updated",
        /// `repo:commit_status_created`
        RepoCommitStatusCreated => "repo:commit_status_created",
        /// `repo:commit_status_updated`
        RepoCommitStatusUpdated => "repo:commit_status_updated",
        /// `pullrequest:created`
        PullRequestCreated => "pullrequest:created",
        /// `pullrequest:updated`
        PullRequestUpdated => "pullrequest:updated",
        /// `pullrequest:approved`
        PullRequestApproved => "pullrequest:approved",
        /// `pullrequest:unapproved`
        PullRequestUnapproved => "pullrequest:unapproved",
        /// `pullrequest:fulfilled`
        PullRequestFulfilled => "pullrequest:fulfilled",
        /// `pullrequest:rejected`
        PullRequestRejected => "pullrequest:rejected",
        /// `pullrequest:comment_created`
        PullRequestCommentCreated => "pullrequest:comment_created",
        /// `issue:created`
        IssueCreated => "issue:created",
        /// `issue:updated`
        IssueUpdated => "issue:updated",
        /// `issue:comment_created`
        IssueCommentCreated => "issue:comment_created",
    }
}

/// Start a Bitbucket webhook endpoint builder.
#[must_use]
pub fn endpoint() -> WebhookBuilder {
    WebhookBuilder::new(Provider::Bitbucket)
}

/// Build a Bitbucket webhook from typed events.
pub fn webhook(
    url: impl Into<String>,
    events: impl IntoIterator<Item = Event>,
) -> Result<Webhook> {
    endpoint().url(url).events(events).build()
}
