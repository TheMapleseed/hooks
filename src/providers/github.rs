//! GitHub repository webhooks — typed events, no payload templates.

use crate::common::{Webhook, WebhookBuilder};
use crate::error::Result;
use crate::provider::Provider;

provider_events! {
    /// Common GitHub webhook event names (`X-GitHub-Event`).
    pub enum Event {
        /// `ping`
        Ping => "ping",
        /// `push`
        Push => "push",
        /// `pull_request`
        PullRequest => "pull_request",
        /// `pull_request_review`
        PullRequestReview => "pull_request_review",
        /// `pull_request_review_comment`
        PullRequestReviewComment => "pull_request_review_comment",
        /// `issues`
        Issues => "issues",
        /// `issue_comment`
        IssueComment => "issue_comment",
        /// `create`
        Create => "create",
        /// `delete`
        Delete => "delete",
        /// `fork`
        Fork => "fork",
        /// `star`
        Star => "star",
        /// `watch`
        Watch => "watch",
        /// `release`
        Release => "release",
        /// `workflow_run`
        WorkflowRun => "workflow_run",
        /// `workflow_job`
        WorkflowJob => "workflow_job",
        /// `check_run`
        CheckRun => "check_run",
        /// `check_suite`
        CheckSuite => "check_suite",
        /// `status`
        Status => "status",
        /// `deployment`
        Deployment => "deployment",
        /// `deployment_status`
        DeploymentStatus => "deployment_status",
        /// `member`
        Member => "member",
        /// `membership`
        Membership => "membership",
        /// `organization`
        Organization => "organization",
        /// `repository`
        Repository => "repository",
        /// `public`
        Public => "public",
        /// `label`
        Label => "label",
        /// `milestone`
        Milestone => "milestone",
        /// `gollum`
        Gollum => "gollum",
        /// `package`
        Package => "package",
        /// `dependabot_alert`
        DependabotAlert => "dependabot_alert",
        /// `secret_scanning_alert`
        SecretScanningAlert => "secret_scanning_alert",
        /// `security_advisory`
        SecurityAdvisory => "security_advisory",
        /// `branch_protection_rule`
        BranchProtectionRule => "branch_protection_rule",
        /// `meta`
        Meta => "meta",
        /// `sponsorship`
        Sponsorship => "sponsorship",
        /// `team`
        Team => "team",
        /// `team_add`
        TeamAdd => "team_add",
    }
}

/// Start a GitHub webhook endpoint builder.
#[must_use]
pub fn endpoint() -> WebhookBuilder {
    WebhookBuilder::new(Provider::GitHub)
}

/// Build a GitHub webhook from typed events.
pub fn webhook(
    url: impl Into<String>,
    events: impl IntoIterator<Item = Event>,
) -> Result<Webhook> {
    endpoint().url(url).events(events).build()
}
