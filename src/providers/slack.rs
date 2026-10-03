//! Slack Events API / interactive payload subscriptions.

use crate::common::{Webhook, WebhookBuilder};
use crate::error::Result;
use crate::provider::Provider;

provider_events! {
    /// Common Slack Events API event types.
    pub enum Event {
        /// `message`
        Message => "message",
        /// `app_mention`
        AppMention => "app_mention",
        /// `app_home_opened`
        AppHomeOpened => "app_home_opened",
        /// `reaction_added`
        ReactionAdded => "reaction_added",
        /// `reaction_removed`
        ReactionRemoved => "reaction_removed",
        /// `member_joined_channel`
        MemberJoinedChannel => "member_joined_channel",
        /// `member_left_channel`
        MemberLeftChannel => "member_left_channel",
        /// `channel_created`
        ChannelCreated => "channel_created",
        /// `channel_rename`
        ChannelRename => "channel_rename",
        /// `channel_archive`
        ChannelArchive => "channel_archive",
        /// `team_join`
        TeamJoin => "team_join",
        /// `file_shared`
        FileShared => "file_shared",
        /// `pin_added`
        PinAdded => "pin_added",
        /// `emoji_changed`
        EmojiChanged => "emoji_changed",
        /// `link_shared`
        LinkShared => "link_shared",
        /// `workflow_step_execute`
        WorkflowStepExecute => "workflow_step_execute",
        /// Interactive component payload (buttons/menus).
        InteractiveComponent => "interactive_component",
        /// Slash command invocation.
        SlashCommand => "slash_command",
        /// Shortcut invocation.
        Shortcut => "shortcut",
        /// URL verification challenge (setup).
        UrlVerification => "url_verification",
    }
}

/// Start a Slack webhook / Events API endpoint builder.
#[must_use]
pub fn endpoint() -> WebhookBuilder {
    WebhookBuilder::new(Provider::Slack)
}

/// Build a Slack endpoint from typed events.
pub fn webhook(
    url: impl Into<String>,
    events: impl IntoIterator<Item = Event>,
) -> Result<Webhook> {
    endpoint().url(url).events(events).build()
}
