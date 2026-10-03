//! Twilio SMS / voice status callback hooks.

use crate::common::{Webhook, WebhookBuilder};
use crate::error::Result;
use crate::provider::Provider;

provider_events! {
    /// Common Twilio callback / webhook event categories.
    pub enum Event {
        /// Inbound SMS/MMS.
        MessagingInbound => "messaging.inbound",
        /// Outbound message status callback.
        MessagingStatusCallback => "messaging.status_callback",
        /// Incoming voice call.
        VoiceIncomingCall => "voice.incoming_call",
        /// Voice status callback.
        VoiceStatusCallback => "voice.status_callback",
        /// Voice recording status.
        VoiceRecordingStatus => "voice.recording_status",
        /// Conversation message added.
        ConversationMessageAdded => "conversation.message_added",
        /// Conversation state updated.
        ConversationStateUpdated => "conversation.state_updated",
        /// Verify verification status.
        VerifyStatus => "verify.status",
    }
}

/// Start a Twilio webhook endpoint builder.
#[must_use]
pub fn endpoint() -> WebhookBuilder {
    WebhookBuilder::new(Provider::Twilio)
}

/// Build a Twilio webhook from typed events.
pub fn webhook(
    url: impl Into<String>,
    events: impl IntoIterator<Item = Event>,
) -> Result<Webhook> {
    endpoint().url(url).events(events).build()
}
